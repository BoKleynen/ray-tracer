use crate::shape::{GeometricObject, AABB};
use crate::world::World;

pub struct ShapeNode<'a> {
    pub(crate) aabb: AABB,
    pub(crate) obj: &'a GeometricObject,
}

pub struct BVH<'a> {
    pub world: &'a World,
    pub root_node: BVHNode,
}

pub struct BVHNode {
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub aabb: AABB,
}
