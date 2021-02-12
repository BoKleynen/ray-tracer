mod multiple_objects;

pub use multiple_objects::MultipleObjects;

use crate::film::RGB;
use crate::math::Ray;

pub trait Tracer: Sync {
    fn trace_ray(&self, ray: &Ray) -> RGB;
}
