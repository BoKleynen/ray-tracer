use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape};
use crate::K_EPSILON;
use nalgebra::{Point3, Unit, Vector3};

pub struct Rectangle {
    p: Point3<f64>,
    a: Vector3<f64>,
    b: Vector3<f64>,
    normal: Unit<Vector3<f64>>,
    transformation: Transformation,
}

impl Rectangle {
    fn shading_normal(&self, normal: &Vector3<f64>) -> Vector3<f64> {
        self.transformation
            .inverse()
            .matrix()
            .transpose()
            .transform_vector(normal)
            .normalize()
    }
}

impl Shape for Rectangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let inv_ray = self.transformation.apply_inverse(ray);

        let t =
            (self.p - inv_ray.origin()).dot(&self.normal) / inv_ray.direction().dot(&self.normal);

        if t <= K_EPSILON {
            return None;
        }

        let q = inv_ray.origin() + t * inv_ray.direction();
        let d = q - self.p;

        let ddota = d.dot(&self.a);
        if ddota < 0. || ddota > self.a.norm_squared() {
            return None;
        }

        let ddotb = d.dot(&self.b);
        if ddotb < 0. || ddotb > self.b.norm_squared() {
            return None;
        }

        Some(Hit {
            t,
            normal: self.shading_normal(&self.normal),
            local_hit_point: q,
        })
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        1
    }
}
