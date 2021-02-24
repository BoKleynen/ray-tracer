mod aabb;
mod cuboid;
mod obj;
mod plane;
mod sphere;

pub use cuboid::Cuboid;
pub use obj::{Obj, TriangleMesh};
pub use plane::Plane;
pub use sphere::Sphere;

use crate::material::Material;
use crate::math::Ray;
use nalgebra::{Point3, Vector3};

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn material(&self) -> Material;

    fn count_intersection_tests(&self, ray: &Ray) -> usize;

    fn hit(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

pub struct Hit {
    pub t: f64,
    pub normal: Vector3<f64>,
    pub local_hit_point: Point3<f64>,
}
