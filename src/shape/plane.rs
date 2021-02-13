use crate::film::RGB;
use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape};
use nalgebra::{Point3, Vector3};

pub struct Plane {
    normal: Vector3<f64>,
    point: Point3<f64>,
    transformation: Transformation,
    material: Material,
}

impl Plane {
    pub fn new(
        normal: Vector3<f64>,
        point: Point3<f64>,
        transformation: Transformation,
        material: Material,
    ) -> Self {
        Self {
            normal,
            point,
            transformation,
            material,
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let inv_ray = self.transformation.apply_inverse(ray);

        let t = ((&self.point - inv_ray.origin()).dot(&self.normal))
            / (inv_ray.direction().dot(&self.normal));

        if t > f64::EPSILON {
            return Some(Hit {
                t,
                normal: Vector3::default(),
                local_hit_point: Point3::origin(),
            });
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material.clone()
    }
}
