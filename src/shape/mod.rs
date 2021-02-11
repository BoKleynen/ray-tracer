mod cuboid;
mod plane;
mod sphere;

use crate::math::homogeneous::Ray;
pub use cuboid::Cuboid;
pub use plane::Plane;
pub use sphere::Sphere;

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &Ray) -> bool;
}
