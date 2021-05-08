use nalgebra::Unit;

use crate::math::Ray;
use crate::sampler::Sample;
use crate::shape::{Hit, Shape, AABB};
use crate::{Point, Vector, K_EPSILON};

#[derive(Clone)]
pub struct Rectangle {
    p: Point,
    a: Vector,
    b: Vector,
    normal: Unit<Vector>,
}

impl Rectangle {
    pub fn new(p: Point, a: Vector, b: Vector) -> Self {
        let normal = Unit::new_normalize(a.cross(&b));
        Self { p, a, b, normal }
    }

    pub fn area(&self) -> f64 {
        self.a.cross(&self.b).norm()
    }

    pub fn sample(&self, sample: &Sample) -> Point {
        self.p + sample.0 * self.a + sample.1 * self.b
    }

    pub fn normal_at(&self, _p: &Point) -> Unit<Vector> {
        self.normal
    }
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
