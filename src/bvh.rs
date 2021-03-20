use crate::bvh::NodeType::{Internal, Leaf};
use crate::math::Ray;
use crate::shape::{GeometricObject, Hit, Shape, AABB};
use crate::world::World;
use itertools::Itertools;
use nalgebra::Point3;
use std::mem::ManuallyDrop;

pub struct ShapeNode<'a> {
    pub(crate) aabb: AABB,
    pub(crate) obj: &'a GeometricObject,
}

pub struct BVH<S> {
    node: Node<S>,
}

impl<S: Shape> BVH<S> {
    pub fn new(mut shapes: Vec<S>) -> Self {
        shapes.sort_unstable_by(|s0, s1| {
            let m0 = s0.bbox().p1.x - s0.bbox().p0.x;
            let m1 = s1.bbox().p1.x - s1.bbox().p0.x;

            m0.partial_cmp(&m1).unwrap()
        });

        let node = Node::new(shapes);
        Self { node }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.node.intersect(ray)
    }

    pub fn bbox(&self) -> AABB {
        self.node.bbox
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
    fn new(mut shapes: Vec<S>) -> Self {
        let bbox = bounding_box(&shapes);
        if shapes.len() <= 3 {
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
                let left_hit = left.bbox.intersect(ray);
                let right_hit = right.bbox.intersect(ray);

                match (left_hit, right_hit) {
                    (Some(left_t), Some(right_t)) => {
                        if left_t < right_t {
                            left.intersect(ray).or_else(|| right.intersect(ray))
                        } else {
                            right.intersect(ray).or_else(|| left.intersect(ray))
                        }
                    }
                    (Some(_), None) => left.intersect(ray),
                    (None, Some(_)) => right.intersect(ray),
                    (None, None) => None,
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
