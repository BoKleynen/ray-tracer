use nalgebra::Point3;
use crate::math::Ray;
use crate::K_EPSILON;

pub struct AABB {
    // p1 > p2
    p1: Point3<f64>,
    p2: Point3<f64>,
}

impl AABB {
    pub fn new(p1: Point3<f64>, p2: Point3<f64>) -> Self {
        Self { p1, p2 }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let ox = ray.origin().x;
        let oy = ray.origin().y;
        let oz = ray.origin().z;

        let dx = ray.direction().x;
        let dy = ray.direction().y;
        let dz = ray.direction().z;

        let a = 1. / dx;
        let (tx_min, tx_max) = if a >= 0. {
            ((self.p2.x - ox) * a, (self.p1.x - ox) * a)
        } else {
            ((self.p1.x - ox) * a, (self.p2.x - ox) * a)
        };

        let b = 1. / dy;
        let (ty_min, ty_max) = if b >= 0. {
            ((self.p2.y - oy) * b, (self.p1.y - oy) * b)
        } else {
            ((self.p1.y - oy) * b, (self.p2.y - oy) * b)
        };

        let c = 1. / dz;
        let (tz_min, tz_max) = if c >= 0. {
            ((self.p2.z - oz) * c, (self.p1.z - oz) * c)
        } else {
            ((self.p1.z - oz) * c, (self.p2.z - oz) * c)
        };

        // find largest entering t value
        let t0 = tx_min.max(ty_min).max(tz_min);

        // find smallest exiting t value
        let t1 = tx_max.min(ty_max).min(tz_max);

        t0 < t1 && t1 > K_EPSILON
    }
}
