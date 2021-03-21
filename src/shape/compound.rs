#[cfg(feature = "bvh")]
use crate::bvh::BVH;
use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};

#[cfg(not(any(feature = "bvh")))]
pub struct Compound<S> {
    shapes: Vec<S>,
    bbox: AABB,
}

#[cfg(not(any(feature = "bvh")))]
impl<S: Shape> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        assert!(shapes.len() > 1);

        let bbox = AABB::from_multiple(shapes.as_slice());

        Self { shapes, bbox }
    }
}

#[cfg(not(any(feature = "bvh")))]
impl<S: Shape> Shape for Compound<S> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        if self.bbox.intersect(ray).is_none() {
            return None;
        }

        self.shapes
            .iter()
            .filter_map(|shape| shape.intersect(ray))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        if self.bbox.intersect(ray).is_none() {
            return 1;
        }

        1 + self
            .shapes
            .iter()
            .map(|shape| shape.count_intersection_tests(ray))
            .sum::<usize>()
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

#[cfg(feature = "bvh")]
pub struct Compound<S> {
    bvh: BVH<S>,
}

#[cfg(feature = "bvh")]
impl<S: Shape> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        assert!(shapes.len() > 1);

        let bvh = BVH::new(shapes);
        Self { bvh }
    }
}

#[cfg(feature = "bvh")]
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
