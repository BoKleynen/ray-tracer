use crate::math::Ray;
use crate::shape::{Aabb, Bounded, Hit, Shape, Intersect};
use crate::Point;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::{addr_of, addr_of_mut};
use NodeKind::*;
use SplittingHeuristic::*;

pub enum SplittingHeuristic {
    SpaceMedianSplit,
    ObjectMedianSplit,
    Sah,
}

pub struct Bvh<'a, S: 'a> {
    shapes: Pin<Box<[S]>>,
    root: Node<'a, S>,
}

#[derive(Copy, Clone)]
struct ShapeData<'a, S> {
    bbox: Aabb,
    centroid: Point,
    shape: &'a S,
}

impl<S> Bounded for ShapeData<'_, S> {
    #[inline]
    fn bbox(&self) -> Aabb {
        self.bbox
    }
}

impl<'a, S: Intersect> Bvh<'a, S> {
    pub fn new(shapes: Vec<S>, splitting_heuristic: SplittingHeuristic) -> Self {
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        // safety: shapes will be read only from here on an therefore wont move.
        let shapes = unsafe { Pin::new_unchecked(shapes.into_boxed_slice()) };

        // Initializing the `shapes` field
        unsafe {
            addr_of_mut!((*ptr).shapes).write(shapes);
        }

        let root = {
            // safety: get a reference to the previously initialized array of shapes
            #[allow(clippy::deref_addrof)]
            let shapes_ref = unsafe { &*addr_of!((*ptr).shapes) };

            let shape_data = shapes_ref
                .iter()
                .map(|shape| {
                    let bbox = shape.bbox();
                    let centroid = bbox.centroid();

                    ShapeData {
                        bbox,
                        centroid,
                        shape,
                    }
                })
                .collect::<Vec<_>>();

            match splitting_heuristic {
                SpaceMedianSplit => Node::space_median_split(shape_data, 0),
                ObjectMedianSplit => Node::object_median_split(shape_data, 0),
                Sah => Node::space_area_heuristic(shape_data, 0),
            }
        };

        // Initializing the `root` field
        unsafe {
            addr_of_mut!((*ptr).root).write(root);
        }

        // All the fields are initialized, so we call `assume_init` to get an initialized Bvh.
        unsafe { uninit.assume_init() }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit<S::Intersection>> {
        self.root.intersect(ray)
    }

    pub fn bbox(&self) -> Aabb {
        self.root.bbox
    }

    pub fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.root.count_intersection_tests(ray)
    }
}

enum NodeKind<'a, S> {
    Leaf {
        shapes: Vec<&'a S>,
    },
    Internal {
        left: Box<Node<'a, S>>,
        right: Box<Node<'a, S>>,
    },
}

struct Node<'a, S> {
    bbox: Aabb,
    node_type: NodeKind<'a, S>,
}

impl<'a, S: Intersect> Node<'a, S> {
    fn space_area_heuristic(_shapes: Vec<ShapeData<'a, S>>, _axis: usize) -> Self {
        todo!()
    }

    fn space_median_split(shapes: Vec<ShapeData<'a, S>>, axis: usize) -> Self {
        debug_assert!(axis < 3);

        let bbox = Aabb::from_multiple(&shapes);

        if shapes.len() <= 2 {
            let shapes = shapes.iter().map(|s| s.shape).collect();
            Self {
                bbox,
                node_type: Leaf { shapes },
            }
        } else {
            let (left, right) = Self::split_space(shapes, axis);

            if left.is_empty() {
                let shapes = right.iter().map(|s| s.shape).collect();
                Self {
                    bbox,
                    node_type: Leaf { shapes },
                }
            } else if right.is_empty() {
                let shapes = left.iter().map(|s| s.shape).collect();
                Self {
                    bbox,
                    node_type: Leaf { shapes },
                }
            } else {
                let next_axis = (axis + 1) % 3;

                Self {
                    bbox,
                    node_type: Internal {
                        left: Box::new(Self::space_median_split(left, next_axis)),
                        right: Box::new(Self::space_median_split(right, next_axis)),
                    },
                }
            }
        }
    }

    fn object_median_split(mut shapes: Vec<ShapeData<'a, S>>, axis: usize) -> Self {
        Self::object_median_split_rec(&mut shapes, axis)
    }

    fn object_median_split_rec(shapes: &mut [ShapeData<'a, S>], axis: usize) -> Self {
        debug_assert!(axis < 3);

        let bbox = Aabb::from_multiple(&shapes);

        if shapes.len() <= 2 {
            let shapes = shapes.iter().map(|s| s.shape).collect();
            Self {
                bbox,
                node_type: Leaf { shapes },
            }
        } else {
            shapes
                .sort_unstable_by(|a, b| a.centroid[axis].partial_cmp(&b.centroid[axis]).unwrap());

            let (left, right) = shapes.split_at_mut(shapes.len() / 2);
            let next_axis = (axis + 1) % 3;

            Self {
                bbox,
                node_type: Internal {
                    left: Box::new(Self::object_median_split_rec(left, next_axis)),
                    right: Box::new(Self::object_median_split_rec(right, next_axis)),
                },
            }
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Hit<S::Intersection>> {
        match &self.node_type {
            Leaf { shapes } => shapes
                .iter()
                .filter_map(|shape| shape.intersect(&ray))
                .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap()),
            Internal { left, right } => {
                match (left.bbox.intersect(ray), right.bbox.intersect(ray)) {
                    (Some(left_t), Some(right_t)) => {
                        if left_t < right_t {
                            Self::intersect_overlapping_bbox(ray, left, right, right_t)
                        } else {
                            Self::intersect_overlapping_bbox(ray, right, left, left_t)
                        }
                    }
                    (Some(_), None) => left.intersect(ray),
                    (None, Some(_)) => right.intersect(ray),
                    (None, None) => None,
                }
            }
        }
    }

    fn intersect_overlapping_bbox(
        ray: &Ray,
        first: &Self,
        second: &Self,
        second_t: f64,
    ) -> Option<Hit<S::Intersection>> {
        match first.intersect(ray) {
            None => second.intersect(ray),
            Some(hit) if second_t <= hit.t => match second.intersect(ray) {
                Some(new_hit) if new_hit.t < hit.t => Some(new_hit),
                _ => Some(hit),
            },
            Some(hit) => Some(hit),
        }
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        match &self.node_type {
            Leaf { shapes } => shapes
                .iter()
                .map(|shape| shape.count_intersection_tests(ray))
                .sum(),
            Internal { left, right } => {
                2 + match (left.bbox.intersect(ray), right.bbox.intersect(ray)) {
                    (Some(left_t), Some(right_t)) => {
                        if left_t < right_t {
                            left.count_intersection_tests(ray)
                                + match left.intersect(ray) {
                                    Some(hit) if hit.t < right_t => 0,
                                    _ => right.count_intersection_tests(ray),
                                }
                        } else {
                            right.count_intersection_tests(ray)
                                + match right.intersect(ray) {
                                    Some(hit) if hit.t >= left_t => 0,
                                    _ => left.count_intersection_tests(ray),
                                }
                        }
                    }
                    (Some(_), None) => left.count_intersection_tests(ray),
                    (None, Some(_)) => right.count_intersection_tests(ray),
                    (None, None) => 0,
                }
            }
        }
    }

    fn split_space(
        shapes: Vec<ShapeData<'a, S>>,
        axis: usize,
    ) -> (Vec<ShapeData<'a, S>>, Vec<ShapeData<'a, S>>) {
        let split = shapes
            .iter()
            .map(|sample| sample.centroid[axis])
            .sum::<f64>()
            / shapes.len() as f64;
        shapes
            .into_iter()
            .partition(|shape| shape.centroid[axis] < split)
    }
}
