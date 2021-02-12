mod cuboid;
mod obj;
mod plane;
mod sphere;

use crate::film::RGB;
use crate::math::Ray;
pub use cuboid::Cuboid;
pub use obj::{Obj, TriangleMesh};
pub use plane::Plane;
pub use sphere::Sphere;

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn color(&self) -> RGB;
}
