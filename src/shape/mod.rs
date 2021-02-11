mod cuboid;
mod sphere;

pub use cuboid::Cuboid;
pub use sphere::Sphere;
use crate::math::homogeneous::Ray;


pub trait Shape {
    fn intersect(&self, ray: &Ray) -> bool;
}
