use crate::core::{Bounded, Ray, SurfaceInteraction};
use crate::Float;

pub trait Shape: Bounded {
    fn intersect(&self, ray: &Ray) -> Option<(Float, SurfaceInteraction)>;

    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn area(&self) -> Float;
}
