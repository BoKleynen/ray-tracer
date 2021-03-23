use crate::bvh::NodeType::{Internal, Leaf};
use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};
use rand::prelude::*;
use std::borrow::BorrowMut;

pub struct BVH<S> {
    node: Node<S>,
}

impl<S: Shape> BVH<S> {
    pub fn new(mut shapes: Vec<S>) -> Self {
        Self {
            node: Node::new(shapes),
        }
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

impl<S: Shape> Node<S> {
    fn new(shapes: Vec<S>) -> Self {
        Self::new_x(shapes)
    }

    fn new_x(shapes: Vec<S>) -> Node<S> {
        let bbox = AABB::from_multiple(&shapes);

        if shapes.len() <= 2 {
            Self {
                bbox,
                node_type: Leaf { shapes },
            }
        } else {
            let (left, right) = Self::split_x(shapes);

            if left.is_empty() {
                Self {
                    bbox,
                    node_type: Leaf { shapes: right },
                }
            } else if right.is_empty() {
                Self {
                    bbox,
                    node_type: Leaf { shapes: left },
                }
            } else {
                Self {
                    bbox,
                    node_type: Internal {
                        left: Box::new(Self::new_y(left)),
                        right: Box::new(Self::new_y(right)),
                    },
                }
            }
        }
    }

    fn new_y(shapes: Vec<S>) -> Node<S> {
        let bbox = AABB::from_multiple(&shapes);

        if shapes.len() <= 2 {
            Self {
                bbox,
                node_type: Leaf { shapes },
            }
        } else {
            let (left, right) = Self::split_y(shapes);


            if left.is_empty() {
                Self {
                    bbox,
                    node_type: Leaf { shapes: right },
                }
            } else if right.is_empty() {
                Self {
                    bbox,
                    node_type: Leaf { shapes: left },
                }
            } else {
                Self {
                    bbox,
                    node_type: Internal {
                        left: Box::new(Self::new_z(left)),
                        right: Box::new(Self::new_z(right)),
                    },
                }
            }
        }
    }

    fn new_z(shapes: Vec<S>) -> Node<S> {
        let bbox = AABB::from_multiple(&shapes);

        if shapes.len() <= 2 {
            Self {
                bbox,
                node_type: Leaf { shapes },
            }
        } else {
            let (left, right) = Self::split_z(shapes);

            if left.is_empty() {
                Self {
                    bbox,
                    node_type: Leaf { shapes: right },
                }
            } else if right.is_empty() {
                Self {
                    bbox,
                    node_type: Leaf { shapes: left },
                }
            } else {
                Self {
                    bbox,
                    node_type: Internal {
                        left: Box::new(Self::new_x(left)),
                        right: Box::new(Self::new_x(right)),
                    },
                }
            }
        }
    }

    fn split_x(shapes: Vec<S>) -> (Vec<S>, Vec<S>) {
        let nb_samples = 20.min(shapes.len() - 1);
        let split = shapes.iter()
            // .choose_multiple(thread_rng().borrow_mut(), nb_samples)
            .map(|sample| sample.bbox().centroid().x)
            .sum::<f64>()
            / shapes.len() as f64;
        shapes
            .into_iter()
            .partition(|shape| shape.bbox().centroid().x < split)
    }

    fn split_y(shapes: Vec<S>) -> (Vec<S>, Vec<S>) {
        let nb_samples = 20.min(shapes.len() - 1);
        let split = shapes.iter()
            // .choose_multiple(thread_rng().borrow_mut(), nb_samples)
            .map(|sample| sample.bbox().centroid().y)
            .sum::<f64>()
            / shapes.len() as f64;
        shapes
            .into_iter()
            .partition(|shape| shape.bbox().centroid().y < split)
    }

    fn split_z(shapes: Vec<S>) -> (Vec<S>, Vec<S>) {
        let nb_samples = 20.min(shapes.len() - 1);
        let split = shapes.iter()
            // .choose_multiple(thread_rng().borrow_mut(), nb_samples)
            .map(|sample| sample.bbox().centroid().z)
            .sum::<f64>()
            / shapes.len() as f64;
        shapes
            .into_iter()
            .partition(|shape| shape.bbox().centroid().z < split)
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
                                Some(hit) => {
                                    if hit.t >= right_t {
                                        match right.intersect(ray) {
                                            Some(right_hit) if hit.t > right_hit.t => {
                                                Some(right_hit)
                                            }
                                            _ => Some(hit),
                                        }
                                    } else {
                                        Some(hit)
                                    }
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
}
