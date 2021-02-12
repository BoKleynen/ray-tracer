mod cuboid;
mod obj;
mod plane;
mod sphere;

use crate::film::RGB;
use crate::math::Ray;
pub use cuboid::Cuboid;
use nalgebra::{Point3, Vector3};
pub use obj::{Obj, TriangleMesh};
pub use plane::Plane;
pub use sphere::Sphere;

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn color(&self) -> RGB;
}

pub struct Hit {
    pub t: f64,
    pub normal: Vector3<f64>,
    pub local_hit_point: Point3<f64>,
}
