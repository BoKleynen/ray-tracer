use crate::film::RGB;
use crate::math::{Ray, Transformation};
use crate::shape::Shape;
use nalgebra::{Point3, Vector3};

pub struct Plane {
    normal: Vector3<f64>,
    point: Point3<f64>,
    transformation: Transformation,
    color: RGB,
}

impl Plane {
    pub fn new(
        normal: Vector3<f64>,
        point: Point3<f64>,
        transformation: Transformation,
        color: RGB,
    ) -> Self {
        Self {
            normal,
            point,
            transformation,
            color,
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let inv_ray = self.transformation.apply_inverse(ray);

        let t = ((&self.point - inv_ray.origin()).dot(&self.normal))
            / (inv_ray.direction().dot(&self.normal));

        if t > f64::EPSILON {
            Some(t)
        } else {
            None
        }
    }

    fn color(&self) -> RGB {
        self.color
    }
}
