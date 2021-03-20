use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape, AABB};
use crate::K_EPSILON;
use nalgebra::{Point3, Vector3};

pub struct Plane {
    normal: Vector3<f64>,
    point: Point3<f64>,
}

impl Plane {
    pub fn new(normal: Vector3<f64>, point: Point3<f64>) -> Self {
        Self { normal, point }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let t =
            ((self.point - ray.origin()).dot(&self.normal)) / (ray.direction().dot(&self.normal));

        if t > K_EPSILON {
            return Some(Hit {
                t,
                normal: self.normal,
                local_hit_point: ray.origin() + t * ray.direction(),
            });
        } else {
            None
        }
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }

    fn bbox(&self) -> AABB {
        AABB::new(
            Point3::new(f64::MIN, f64::MIN, f64::MIN),
            Point3::new(f64::MAX, f64::MAX, f64::MAX),
        )
    }
}
