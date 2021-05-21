use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::{addr_of, addr_of_mut};

use crate::bvh::AxisSelection::*;
use crate::math::Ray;
use crate::shape::{Aabb, Bounded, Hit, Intersect, Union};
use crate::Point3;
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};
use NodeKind::*;
use SplittingHeuristic::*;

pub const X_AXIS: usize = 0;
pub const Y_AXIS: usize = 1;
pub const Z_AXIS: usize = 2;

const DIRECTIONS: [usize; 3] = [X_AXIS, Y_AXIS, Z_AXIS];

#[derive(Debug, Copy, Clone, Default)]
pub struct SplittingConfig {
    pub splitting_heuristic: SplittingHeuristic,
    pub axis_selection: AxisSelection,
}

impl Display for SplittingConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.axis_selection, self.splitting_heuristic)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SplittingHeuristic {
    SpaceMedianSplit,
    ObjectMedianSplit,
    SpaceAverageSplit,
    SurfaceAreaHeuristic(usize),
}

impl Default for SplittingHeuristic {
    fn default() -> Self {
        SurfaceAreaHeuristic(12)
    }
}

impl Display for SplittingHeuristic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SpaceMedianSplit => write!(f, "SpaceMedianSplit"),
            ObjectMedianSplit => write!(f, "ObjectMedianSplit"),
            SpaceAverageSplit => write!(f, "SpaceAverageSplit"),
            SurfaceAreaHeuristic(nb_buckets) => write!(f, "SurfaceAreaHeuristic({})", nb_buckets),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AxisSelection {
    Alternate(usize),
    Longest,
}

impl Default for AxisSelection {
    fn default() -> Self {
        Alternate(X_AXIS)
    }
}

impl Display for AxisSelection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Alternate(_) => write!(f, "Alternate"),
            Longest => write!(f, "Longest"),
        }
    }
}

#[derive(Debug)]
pub struct Bvh<'a, S: 'a> {
    shapes: Pin<Box<[S]>>,
    root: Node<'a, S>,
}

impl<'a, S: Intersect> Bvh<'a, S> {
    pub fn new(shapes: Vec<S>, cfg: SplittingConfig) -> Self {
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        // safety: shapes will be read only from here on an therefore wont move.
        let shapes = unsafe { Pin::new_unchecked(shapes.into_boxed_slice()) };
        // Initializing the `shapes` field.
        unsafe {
            addr_of_mut!((*ptr).shapes).write(shapes);
        }

        let root = {
            // safety: get a reference to the previously initialized array of shapes.
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
                .collect();

            match cfg.splitting_heuristic {
                SpaceMedianSplit => Node::space_median_split(shape_data, 3, cfg.axis_selection),
                ObjectMedianSplit => Node::object_median_split(shape_data, cfg.axis_selection),
                SpaceAverageSplit => unimplemented!(),
                SurfaceAreaHeuristic(nb_buckets) => {
                    Node::surface_area_heuristic(shape_data, nb_buckets, cfg.axis_selection)
                }
            }
        };

        // Initializing the `root` field.
        unsafe {
            addr_of_mut!((*ptr).root).write(root);
        }

        // All the fields are initialized, so we call `assume_init` to get an initialized Bvh.
        unsafe { uninit.assume_init() }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit<S::Intersection>> {
        self.root.intersect(ray)
    }

    pub fn intersect_any_where<F>(&self, ray: &Ray, f: F) -> bool
    where
        F: Fn(Hit<S::Intersection>) -> bool,
    {
        self.root.intersect_any_where(ray, &f)
    }

    pub fn bbox(&self) -> Aabb {
        self.root.bbox
    }

    pub fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.root.count_intersection_tests(ray)
    }
}

#[derive(Debug, Copy, Clone)]
struct ShapeData<'a, S> {
    bbox: Aabb,
    centroid: Point3,
    shape: &'a S,
}

impl<S> Bounded for ShapeData<'_, S> {
    fn bbox(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Debug)]
enum NodeKind<'a, S> {
    Leaf {
        shapes: Vec<&'a S>,
    },
    Internal {
        left: Box<Node<'a, S>>,
        right: Box<Node<'a, S>>,
    },
}

#[derive(Debug)]
struct Node<'a, S> {
    bbox: Aabb,
    node_kind: NodeKind<'a, S>,
}

impl<'a, S: Intersect> Node<'a, S> {
    fn leaf(bbox: Aabb, shapes: Vec<ShapeData<'a, S>>) -> Self {
        let shapes = shapes.iter().map(|s| s.shape).collect();
        Self {
            bbox,
            node_kind: Leaf { shapes },
        }
    }

    fn surface_area_heuristic(
        shapes: Vec<ShapeData<'a, S>>,
        nb_buckets: usize,
        axis_selection: AxisSelection,
    ) -> Self {
        match axis_selection {
            Alternate(prev_axis) => {
                Self::surface_area_heuristic_alternate(shapes, nb_buckets, prev_axis)
            }
            Longest => Self::surface_area_heuristic_longest(shapes, nb_buckets, &DIRECTIONS),
        }
    }

    fn surface_area_heuristic_alternate(
        shapes: Vec<ShapeData<'a, S>>,
        nb_buckets: usize,
        prev_axis: usize,
    ) -> Self {
        let bbox = Aabb::from_multiple(&shapes);
        if shapes.len() <= 2 {
            Self::leaf(bbox, shapes)
        } else {
            let axis = (prev_axis + 1) % 3;
            let split_axis_size = bbox.p1[axis] - bbox.p0[axis];

            Self::surface_area_heuristic_inner(
                shapes,
                nb_buckets,
                axis,
                split_axis_size,
                bbox,
                |shapes| Self::surface_area_heuristic_alternate(shapes, nb_buckets, axis),
            )
        }
    }

    fn surface_area_heuristic_longest(
        shapes: Vec<ShapeData<'a, S>>,
        nb_buckets: usize,
        directions: &[usize],
    ) -> Self {
        let bbox = Aabb::from_multiple(&shapes);
        if shapes.len() <= 2 || directions.is_empty() {
            Self::leaf(bbox, shapes)
        } else {
            let mut buckets = vec![BucketInfo::default(); nb_buckets];
            let (axis, axis_size) = directions
                .iter()
                .map(|&axis| (axis, bbox.p1[axis] - bbox.p0[axis]))
                .max_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap())
                .unwrap();

            shapes.iter().for_each(|shape| {
                let b = nb_buckets as f64 * (shape.centroid[axis] - bbox.p0[axis]) / axis_size;
                let b = b.floor() as usize;

                buckets[b].count += 1;
                buckets[b].bbox = buckets[b].bbox.union(shape.bbox);
            });

            let (min_bucket, min_cost) = (0..nb_buckets - 1)
                .map(|i| {
                    let left_count = buckets[..=i]
                        .iter()
                        .map(|bucket| bucket.count)
                        .sum::<usize>();
                    let left_bbox = buckets[..=i]
                        .iter()
                        .fold(Aabb::default(), |acc, bucket| acc.union(bucket.bbox));

                    let right_count = buckets[i + 1..]
                        .iter()
                        .map(|bucket| bucket.count)
                        .sum::<usize>();
                    let right_bbox = buckets[i + 1..]
                        .iter()
                        .fold(Aabb::default(), |acc, bucket| acc.union(bucket.bbox));

                    let cost = 1.
                        + (left_count as f64 * left_bbox.surface()
                            + right_count as f64 * right_bbox.surface())
                            / bbox.surface();

                    (i, cost)
                })
                .min_by(|(_, c1), (_, c2)| c1.partial_cmp(&c2).unwrap())
                .unwrap();

            let leaf_cost = shapes.len() as f64;
            if leaf_cost <= min_cost {
                Self::surface_area_heuristic_longest(
                    shapes,
                    nb_buckets,
                    &directions
                        .iter()
                        .copied()
                        .filter(|&dir| dir != axis)
                        .collect_vec(),
                )
            } else {
                let (left, right): (Vec<_>, Vec<_>) = shapes.into_iter().partition(|shape| {
                    let b = nb_buckets as f64 * (shape.centroid[axis] - bbox.p0[axis]) / axis_size;
                    let b = b.floor() as usize;

                    b <= min_bucket
                });

                Self {
                    bbox,
                    node_kind: Internal {
                        left: Box::new(Self::surface_area_heuristic_longest(
                            left,
                            nb_buckets,
                            &DIRECTIONS,
                        )),
                        right: Box::new(Self::surface_area_heuristic_longest(
                            right,
                            nb_buckets,
                            &DIRECTIONS,
                        )),
                    },
                }
            }
        }
    }

    fn surface_area_heuristic_inner<F>(
        shapes: Vec<ShapeData<'a, S>>,
        nb_buckets: usize,
        axis: usize,
        axis_size: f64,
        bbox: Aabb,
        recursive: F,
    ) -> Self
    where
        F: Fn(Vec<ShapeData<'a, S>>) -> Self + Copy,
    {
        let mut buckets = vec![BucketInfo::default(); nb_buckets];
        shapes.iter().for_each(|shape| {
            let b = nb_buckets as f64 * (shape.centroid[axis] - bbox.p0[axis]) / axis_size;
            let b = (b.floor() as usize).min(nb_buckets - 1);

            buckets[b].count += 1;
            buckets[b].bbox = buckets[b].bbox.union(shape.bbox);
        });

        let (min_bucket, min_cost) = (0..nb_buckets - 1)
            .map(|i| {
                let left_count = buckets[..=i]
                    .iter()
                    .map(|bucket| bucket.count)
                    .sum::<usize>();
                let left_bbox = buckets[..=i]
                    .iter()
                    .fold(Aabb::default(), |acc, bucket| acc.union(bucket.bbox));

                let right_count = buckets[i + 1..]
                    .iter()
                    .map(|bucket| bucket.count)
                    .sum::<usize>();
                let right_bbox = buckets[i + 1..]
                    .iter()
                    .fold(Aabb::default(), |acc, bucket| acc.union(bucket.bbox));

                let cost = 1.
                    + (left_count as f64 * left_bbox.surface()
                        + right_count as f64 * right_bbox.surface())
                        / bbox.surface();

                (i, cost)
            })
            .min_by(|(_, c1), (_, c2)| c1.partial_cmp(&c2).unwrap())
            .unwrap();

        let leaf_cost = shapes.len() as f64;
        if leaf_cost <= min_cost {
            Self::leaf(bbox, shapes)
        } else {
            let (left, right): (Vec<_>, Vec<_>) = shapes.into_iter().partition(|shape| {
                let b = nb_buckets as f64 * (shape.centroid[axis] - bbox.p0[axis]) / axis_size;
                let b = b.floor() as usize;

                b <= min_bucket
            });

            Self {
                bbox,
                node_kind: Internal {
                    left: Box::new(recursive(left)),
                    right: Box::new(recursive(right)),
                },
            }
        }
    }

    fn space_median_split(
        shapes: Vec<ShapeData<'a, S>>,
        counter: u8,
        axis_selection: AxisSelection,
    ) -> Self {
        let bbox = Aabb::from_multiple(&shapes);

        if shapes.len() <= 2 || counter == 0 {
            Self::leaf(bbox, shapes)
        } else {
            match axis_selection {
                Alternate(prev_exis) => {
                    let axis = (prev_exis + 1) % 3;
                    let median = bbox.p0[axis] + (bbox.p1[axis] - bbox.p0[axis]) / 2.;
                    let (left, right) = Self::split_space(shapes, axis, median);

                    if left.is_empty() {
                        Self::space_median_split(right, counter - 1, Alternate(axis))
                    } else if right.is_empty() {
                        Self::space_median_split(left, counter - 1, Alternate(axis))
                    } else {
                        Self {
                            bbox,
                            node_kind: Internal {
                                left: Box::new(Self::space_median_split(left, 3, Alternate(axis))),
                                right: Box::new(Self::space_median_split(
                                    right,
                                    3,
                                    Alternate(axis),
                                )),
                            },
                        }
                    }
                }
                Longest => {
                    let (axis, _) = DIRECTIONS
                        .iter()
                        .map(|&axis| (axis, bbox.p1[axis] - bbox.p0[axis]))
                        .max_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap())
                        .unwrap();

                    let median = bbox.p0[axis] + (bbox.p1[axis] - bbox.p0[axis]) / 2.;
                    let (left, right) = Self::split_space(shapes, axis, median);

                    if left.is_empty() {
                        Self::space_median_split(right, 0, Longest)
                    } else if right.is_empty() {
                        Self::space_median_split(left, 0, Longest)
                    } else {
                        Self {
                            bbox,
                            node_kind: Internal {
                                left: Box::new(Self::space_median_split(left, 1, Longest)),
                                right: Box::new(Self::space_median_split(right, 1, Longest)),
                            },
                        }
                    }
                }
            }
        }
    }

    // fn space_average_split(shapes: Vec<ShapeData<'a, S>>, axis: usize, counter: u8) -> Self {
    //     let bbox = Aabb::from_multiple(&shapes);
    //
    //     if shapes.len() <= 2 || counter == 0 {
    //         Self::leaf(bbox, shapes)
    //     } else {
    //         let next_axis = (axis + 1) % 3;
    //         let (left, right) = Self::split_space_average(shapes, axis);
    //
    //         if left.is_empty() {
    //             Self::space_average_split(right, next_axis, 3)
    //         } else if right.is_empty() {
    //             Self::space_average_split(left, next_axis, 3)
    //         } else {
    //             Self {
    //                 bbox,
    //                 node_kind: Internal {
    //                     left: Box::new(Self::space_average_split(left, next_axis, 3)),
    //                     right: Box::new(Self::space_average_split(right, next_axis, 3)),
    //                 },
    //             }
    //         }
    //     }
    // }

    fn object_median_split(
        mut shapes: Vec<ShapeData<'a, S>>,
        axis_selection: AxisSelection,
    ) -> Self {
        Self::object_median_split_rec(&mut shapes, axis_selection)
    }

    fn object_median_split_rec(
        shapes: &mut [ShapeData<'a, S>],
        axis_selection: AxisSelection,
    ) -> Self {
        let bbox = Aabb::from_multiple(&shapes);

        if shapes.len() <= 2 {
            let shapes = shapes.iter().map(|s| s.shape).collect();
            Self {
                bbox,
                node_kind: Leaf { shapes },
            }
        } else {
            match axis_selection {
                Alternate(prev_axis) => {
                    let axis = (prev_axis + 1) % 3;
                    shapes.sort_unstable_by(|a, b| {
                        a.centroid[axis].partial_cmp(&b.centroid[axis]).unwrap()
                    });

                    let (left, right) = shapes.split_at_mut(shapes.len() / 2);

                    Self {
                        bbox,
                        node_kind: Internal {
                            left: Box::new(Self::object_median_split_rec(left, Alternate(axis))),
                            right: Box::new(Self::object_median_split_rec(right, Alternate(axis))),
                        },
                    }
                }
                Longest => {
                    let (axis, _) = DIRECTIONS
                        .iter()
                        .map(|&axis| (axis, bbox.p1[axis] - bbox.p0[axis]))
                        .max_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap())
                        .unwrap();

                    shapes.sort_unstable_by(|a, b| {
                        a.centroid[axis].partial_cmp(&b.centroid[axis]).unwrap()
                    });

                    let (left, right) = shapes.split_at_mut(shapes.len() / 2);

                    Self {
                        bbox,
                        node_kind: Internal {
                            left: Box::new(Self::object_median_split_rec(left, Alternate(axis))),
                            right: Box::new(Self::object_median_split_rec(right, Alternate(axis))),
                        },
                    }
                }
            }
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Hit<S::Intersection>> {
        match &self.node_kind {
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
        match &self.node_kind {
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

    fn intersect_any_where<F>(&self, ray: &Ray, f: &F) -> bool
    where
        F: Fn(Hit<S::Intersection>) -> bool,
    {
        match &self.node_kind {
            Leaf { shapes } => shapes
                .iter()
                .any(|shape| shape.intersect(ray).map_or(false, |hit| f(hit))),
            Internal { left, right } => {
                match (left.bbox.intersect(ray), right.bbox.intersect(ray)) {
                    (Some(left_t), Some(right_t)) => {
                        // TODO: Optimize this if needed
                        if left_t < right_t {
                            left.intersect_any_where(ray, f) || right.intersect_any_where(ray, f)
                        } else {
                            right.intersect_any_where(ray, f) || left.intersect_any_where(ray, f)
                        }
                    }
                    (Some(_), None) => left.intersect_any_where(ray, f),
                    (None, Some(_)) => right.intersect_any_where(ray, f),
                    (None, None) => false,
                }
            }
        }
    }

    // fn split_space_average(
    //     shapes: Vec<ShapeData<'a, S>>,
    //     axis: usize,
    // ) -> (Vec<ShapeData<'a, S>>, Vec<ShapeData<'a, S>>) {
    //     let split = shapes
    //         .iter()
    //         .map(|sample| sample.centroid[axis])
    //         .sum::<f64>()
    //         / shapes.len() as f64;
    //
    //     Self::split_space(shapes, axis, split)
    // }

    fn split_space(
        shapes: Vec<ShapeData<'a, S>>,
        axis: usize,
        split: f64,
    ) -> (Vec<ShapeData<'a, S>>, Vec<ShapeData<'a, S>>) {
        shapes
            .into_iter()
            .partition(|shape| shape.centroid[axis] < split)
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct BucketInfo {
    count: usize,
    bbox: Aabb,
}
