mod sphere;

use crate::math::homogeneous::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}
