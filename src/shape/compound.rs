#[cfg(feature = "bvh")]
use crate::accel::bvh::{Bvh, SplittingHeuristic};
use crate::math::Ray;
use crate::shape::{Aabb, Bounded, Hit, Intersect};

#[cfg(not(any(feature = "bvh")))]
pub struct Compound<S> {
    shapes: Vec<S>,
    bbox: Aabb,
}

#[cfg(not(any(feature = "bvh")))]
impl<S: Bounded> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        assert!(shapes.len() > 1);

        let bbox = Aabb::from_multiple(shapes.as_slice());

        Self { shapes, bbox }
    }
}

#[cfg(not(any(feature = "bvh")))]
impl<S: Bounded> Bounded for Compound<S> {
    fn bbox(&self) -> Aabb {
        self.bbox
    }
}

#[cfg(not(any(feature = "bvh")))]
impl<S: Intersect> Intersect for Compound<S> {
    type Intersection = S::Intersection;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>> {
        self.bbox.intersect(ray)?;

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
}

#[cfg(not(any(feature = "bvh")))]
impl<S: Intersect> Compound<S> {
    pub fn intersect_any_where<P>(&self, ray: &Ray, p: P) -> bool
    where
        P: Fn(Hit<S::Intersection>) -> bool,
    {
        self.shapes
            .iter()
            .any(|shape| shape.intersect(ray).map_or(false, |hit| p(hit)))
    }
}

#[cfg(feature = "bvh")]
pub struct Compound<S: 'static> {
    bvh: Bvh<'static, S>,
}

#[cfg(feature = "bvh")]
impl<S: Intersect> Compound<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        Self::new_with_splitting_heuristic(shapes, SplittingHeuristic::default())
    }

    pub fn new_with_splitting_heuristic(
        shapes: Vec<S>,
        splitting_heuristic: SplittingHeuristic,
    ) -> Self {
        assert!(!shapes.is_empty());

        Self {
            bvh: Bvh::new(shapes, splitting_heuristic),
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

#[cfg(feature = "bvh")]
impl<S: Intersect> Compound<S> {
    pub fn intersect_any_where<F>(&self, ray: &Ray, f: F) -> bool
    where
        F: Fn(Hit<S::Intersection>) -> bool,
    {
        self.bvh.intersect_any_where(ray, f)
    }
}
