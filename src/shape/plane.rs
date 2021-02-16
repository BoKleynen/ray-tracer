use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape};
use crate::K_EPSILON;
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

    fn shading_normal(&self, normal: &Vector3<f64>) -> Vector3<f64> {
        self.transformation
            .inverse()
            .matrix()
            .transpose()
            .transform_vector(normal)
            .normalize()
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let inv_ray = self.transformation.apply_inverse(ray);

        let t = ((self.point - inv_ray.origin()).dot(&self.normal))
            / (inv_ray.direction().dot(&self.normal));

        if t > K_EPSILON {
            return Some(Hit {
                t,
                normal: self.shading_normal(&self.normal),
                local_hit_point: inv_ray.origin() + t * inv_ray.direction(),
            });
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material.clone()
    }
}
