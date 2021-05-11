#[cfg(feature = "bvh")]
use crate::bvh::Bvh;
use crate::bvh::SplittingHeuristic::*;
use crate::math::Ray;
use crate::shape::{Aabb, Bounded, Hit, Intersect};

#[cfg(not(any(feature = "bvh")))]
pub struct Compound<S> {
    shapes: Vec<S>,
    bbox: Aabb,
}

#[cfg(not(any(feature = "bvh")))]
impl<S: Shape> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        assert!(shapes.len() > 1);

        let bbox = Aabb::from_multiple(shapes.as_slice());

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

    fn bbox(&self) -> Aabb {
        self.bbox
    }
}

#[cfg(feature = "bvh")]
pub struct Compound<S: 'static> {
    bvh: Bvh<'static, S>,
}

#[cfg(feature = "bvh")]
impl<S: Intersect> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        assert!(!shapes.is_empty());

        Self {
            bvh: Bvh::new(shapes, SpaceAverageSplit),
        }
    }
}

#[cfg(feature = "bvh")]
impl<S: Intersect> Bounded for Compound<S> {
    fn bbox(&self) -> Aabb {
        self.bvh.bbox()
    }
}

#[cfg(feature = "bvh")]
impl<S: Intersect> Intersect for Compound<S> {
    type Intersection = S::Intersection;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>> {
        self.bvh.intersect(ray)
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.bvh.count_intersection_tests(ray)
    }
}
