use crate::math::Ray;
use crate::shape::{Aabb, Bounded, Hit, Intersect};
use crate::{Point3, Vector, K_EPSILON, Point2};

pub struct Plane {
    normal: Vector,
    point: Point3,
}

impl Plane {
    pub fn new(normal: Vector, point: Point3) -> Self {
        Self { normal, point }
    }
}

impl Bounded for Plane {
    fn bbox(&self) -> Aabb {
        Aabb::new(
            Point3::new(f64::MIN, f64::MIN, f64::MIN),
            Point3::new(f64::MAX, f64::MAX, f64::MAX),
        )
    }
}

impl Intersect for Plane {
    type Intersection = ();

    fn intersect(&self, ray: &Ray) -> Option<Hit<()>> {
        let t =
            ((self.point - ray.origin()).dot(&self.normal)) / (ray.direction().dot(&self.normal));

        if t > K_EPSILON {
            return Some(Hit {
                t,
                normal: self.normal,
                local_hit_point: ray.origin() + t * ray.direction(),
                shape: (),
                uv: Point2::origin()
            });
        } else {
            None
        }
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }
}
