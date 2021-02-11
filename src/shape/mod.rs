mod cuboid;
mod sphere;

use crate::math::homogeneous::Ray;
pub use cuboid::Cuboid;
pub use sphere::Sphere;

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &Ray) -> bool;
}
