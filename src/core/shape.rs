use crate::core::{Bounded, Ray, SurfaceInteraction};

pub trait Shape: Bounded {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;

    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}
