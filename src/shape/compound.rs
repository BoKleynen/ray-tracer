use crate::bvh::BVH;
use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};
use itertools::Itertools;
use nalgebra::Point3;

pub struct Compound<S> {
    // shapes: Vec<S>,
    bvh: BVH<S>,
}

impl<S: Shape> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        assert!(shapes.len() > 1);

        let bvh = BVH::new(shapes);
        Self { bvh }
    }
}

impl<S: Shape> Shape for Compound<S> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.bvh.intersect(ray)
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.bvh.count_intersection_tests(ray)
    }

    fn bbox(&self) -> AABB {
        self.bvh.bbox()
    }
}
