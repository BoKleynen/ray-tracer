mod multiple_objects;
mod ray_cast;

pub use multiple_objects::MultipleObjects;
pub use ray_cast::RayCast;

use crate::film::RGB;
use crate::math::Ray;

pub trait Tracer: Sync {
    fn trace_ray(&self, ray: &Ray, depth: u32) -> RGB;
}
