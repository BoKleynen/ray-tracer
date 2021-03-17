use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};

pub struct Compound<S> {
    shapes: Vec<S>,
}

impl<S> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        Self { shapes }
    }
}

impl<S: Shape> Shape for Compound<S> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shapes
            .iter()
            .filter_map(|shape| shape.intersect(&ray))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.shapes.len()
    }
}
