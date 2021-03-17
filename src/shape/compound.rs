use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};
use itertools::Itertools;
use nalgebra::Point3;

pub struct Compound<S> {
    shapes: Vec<S>,
}

impl<S> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        assert!(shapes.len() > 1);

        Self { shapes }
    }
}

impl<S: Shape> Compound<S> {
    fn bounding_boxes(&self) -> Vec<AABB> {
        self.shapes.iter().map(Shape::bounding_box).collect()
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

    fn bounding_box(&self) -> AABB {
        let bounding_boxes = self.bounding_boxes();
        let min_x = bounding_boxes
            .iter()
            .map(|aabb| aabb.p0.x)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_x = bounding_boxes
            .iter()
            .map(|aabb| aabb.p1.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let min_y = bounding_boxes
            .iter()
            .map(|aabb| aabb.p0.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_y = bounding_boxes
            .iter()
            .map(|aabb| aabb.p1.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let min_z = bounding_boxes
            .iter()
            .map(|aabb| aabb.p0.z)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_z = bounding_boxes
            .iter()
            .map(|aabb| aabb.p1.z)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        AABB::new(
            Point3::new(min_x, min_y, min_z),
            Point3::new(max_x, max_y, max_z),
        )
    }
}
