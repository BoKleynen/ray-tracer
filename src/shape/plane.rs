use crate::math::Ray;
use crate::shape::{Bounded, Hit, Shape, AABB};
use crate::{Point, Vector, K_EPSILON};

pub struct Plane {
    normal: Vector,
    point: Point,
}

impl Plane {
    pub fn new(normal: Vector, point: Point) -> Self {
        Self { normal, point }
    }
}

impl Bounded for Plane {
    fn bbox(&self) -> AABB {
        AABB::new(
            Point::new(f64::MIN, f64::MIN, f64::MIN),
            Point::new(f64::MAX, f64::MAX, f64::MAX),
        )
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
}
