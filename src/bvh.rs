use crate::bvh::NodeType::{Internal, Leaf};
use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};
use nalgebra::Point3;

pub struct BVH<S> {
    node: Node<S>,
}

impl<S: Shape> BVH<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        // let bboxes = shapes.iter().map(|shape| (shape.bbox(), shape as *const S)).collect_vec();

        let node = Node::new(shapes);
        Self { node }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.node.intersect(ray)
    }

    pub fn bbox(&self) -> AABB {
        self.node.bbox
    }

    pub fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.node.count_intersection_tests(ray)
    }
}

enum NodeType<S> {
    Leaf {
        shapes: Vec<S>,
    },
    Internal {
        left: Box<Node<S>>,
        right: Box<Node<S>>,
    },
}

struct Node<S> {
    bbox: AABB,
    node_type: NodeType<S>,
}

// impl<S: Shape> Node<S> {
//     fn new(bboxes: Vec<(AABB, *const S)>) -> Self {
//
//     }
// }
//
// fn bounding_box<T>(shapes: &[(AABB, T)]) -> AABB {
//     let min_x = shapes
//         .iter()
//         .map(|(bbox,_)| bbox.p0.x)
//         .min_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//     let max_x = shapes
//         .iter()
//         .map(|(bbox,_)| bbox.p1.x)
//         .max_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//     let min_y = shapes
//         .iter()
//         .map(|(bbox,_)| bbox.p0.y)
//         .min_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//     let max_y = shapes
//         .iter()
//         .map(|(bbox,_)| bbox.p1.y)
//         .max_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//     let min_z = shapes
//         .iter()
//         .map(|(bbox,_)| bbox.p0.z)
//         .min_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//     let max_z = shapes
//         .iter()
//         .map(|(bbox,_)| bbox.p1.z)
//         .max_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//
//     AABB::new(
//         Point3::new(min_x, min_y, min_z),
//         Point3::new(max_x, max_y, max_z),
//     )
// }

impl<S: Shape> Node<S> {
    fn new(mut shapes: Vec<S>) -> Self {
        let bbox = bounding_box(&shapes);
        if shapes.len() <= 2 {
            Self {
                bbox,
                node_type: Leaf { shapes },
            }
        } else {
            let right = shapes.split_off(shapes.len() / 2);

            Self {
                bbox,
                node_type: Internal {
                    left: Box::new(Self::new(shapes)),
                    right: Box::new(Self::new(right)),
                },
            }
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        match &self.node_type {
            NodeType::Leaf { shapes } => shapes
                .iter()
                .filter_map(|shape| shape.intersect(&ray))
                .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap()),
            NodeType::Internal { left, right } => {
                match (left.bbox.intersect(ray), right.bbox.intersect(ray)) {
                    (Some(left_t), Some(right_t)) => {
                        if left_t < right_t {
                            match left.intersect(ray) {
                                None => right.intersect(ray),
                                Some(hit) =>
                                    if hit.t >= right_t  {
                                        match right.intersect(ray) {
                                            Some(right_hit) if hit.t > right_hit.t => Some(right_hit),
                                            _ => Some(hit),
                                        }
                                    } else {
                                        Some(hit)
                                    }
                            }
                        } else {
                            match right.intersect(ray) {
                                None => left.intersect(ray),
                                Some(hit) => {
                                    if hit.t >= left_t {
                                        match left.intersect(ray) {
                                            Some(left_hit) if hit.t > left_hit.t => Some(left_hit),
                                            _ => Some(hit),
                                        }
                                    } else {
                                        Some(hit)
                                    }
                                }
                            }
                        }
                    }
                    (Some(_), None) => left.intersect(ray),
                    (None, Some(_)) => right.intersect(ray),
                    (None, None) => None,
                }
            }
        }
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        match &self.node_type {
            NodeType::Leaf { shapes } => shapes
                .iter()
                .map(|shape| shape.count_intersection_tests(ray))
                .sum(),
            NodeType::Internal { left, right } => {
                match (left.bbox.intersect(ray), right.bbox.intersect(ray)) {
                    (Some(left_t), Some(right_t)) => {
                        if left_t < right_t {
                            match left.intersect(ray) {
                                Some(hit) if hit.t < right_t => {
                                    2 + left.count_intersection_tests(ray)
                                }
                                _ => {
                                    2 + left.count_intersection_tests(ray)
                                        + right.count_intersection_tests(ray)
                                }
                            }
                        } else {
                            match right.intersect(ray) {
                                Some(hit) if hit.t >= left_t => {
                                    2 + right.count_intersection_tests(ray)
                                }
                                _ => {
                                    2 + left.count_intersection_tests(ray)
                                        + right.count_intersection_tests(ray)
                                }
                            }
                        }
                    }
                    (Some(_), None) => 2 + left.count_intersection_tests(ray),
                    (None, Some(_)) => 2 + right.count_intersection_tests(ray),
                    (None, None) => 2,
                }
            }
        }
    }
}

fn bounding_box<S: Shape>(shapes: &[S]) -> AABB {
    let min_x = shapes
        .iter()
        .map(|shape| shape.bbox().p0.x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_x = shapes
        .iter()
        .map(|shape| shape.bbox().p1.x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y = shapes
        .iter()
        .map(|shape| shape.bbox().p0.y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_y = shapes
        .iter()
        .map(|shape| shape.bbox().p1.y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_z = shapes
        .iter()
        .map(|shape| shape.bbox().p0.z)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_z = shapes
        .iter()
        .map(|shape| shape.bbox().p1.z)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    AABB::new(
        Point3::new(min_x, min_y, min_z),
        Point3::new(max_x, max_y, max_z),
    )
}
