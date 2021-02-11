use crate::math::homogeneous::{Point, Ray, Transformation, Vector};
use crate::shape::Shape;
use std::f64;

pub struct Plane {
    normal: Vector,
    point: Point,
    transformation: Transformation,
}

impl Plane {
    pub fn new(normal: Vector, point: Point, transformation: Transformation) -> Self {
        Self {
            normal,
            point,
            transformation,
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> bool {
        let inv_ray = self.transformation.apply_inverse(ray);

        let t = (&(&self.point - inv_ray.origin()) * &self.normal)
            / (inv_ray.direction() * &self.normal);
        t > f64::EPSILON
    }
}
