use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};
use crate::K_EPSILON;
use nalgebra::{Point3, Unit, Vector3};

pub struct Rectangle {
    p: Point3<f64>,
    a: Vector3<f64>,
    b: Vector3<f64>,
    normal: Unit<Vector3<f64>>,
}

impl Shape for Rectangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let t = (self.p - ray.origin()).dot(&self.normal) / ray.direction().dot(&self.normal);

        if t <= K_EPSILON {
            return None;
        }

        let q = ray.origin() + t * ray.direction();
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
            normal: *self.normal,
            local_hit_point: q,
        })
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }

    fn bbox(&self) -> AABB {
        let p0 = self.p;
        let p1 = self.p + self.a + self.b;

        AABB::new(p0, p1)
    }
}
