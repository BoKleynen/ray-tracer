use crate::math::Ray;
use crate::shape::{Hit, Shape, AABB};
use crate::K_EPSILON;
use nalgebra::Point3;

/// Represents a three-dimensional unit sphere, centered at the origin,
/// which is transformed by a transformation.
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere::default()
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let origin = ray.origin();
        let direction = ray.direction();

        let a = direction.norm_squared();
        let b = 2. * &origin.coords.dot(direction);
        let c = origin.coords.dot(&origin.coords) - 1.;
        let disc = b * b - 4. * a * c;

        if disc < 0. {
            return None;
        }

        let e = disc.sqrt();
        let denom = 2.0 * a;

        let t = (-b - e) / denom; // smaller root
        if t > K_EPSILON {
            let local_hit_point = origin + t * direction;

            return Some(Hit {
                t,
                normal: local_hit_point.coords,
                local_hit_point,
            });
        }

        let t = (-b + e) / denom; // larger root
        if t > K_EPSILON {
            let local_hit_point = origin + t * direction;

            return Some(Hit {
                t,
                normal: local_hit_point.coords,
                local_hit_point,
            });
        }

        None
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }

    fn bbox(&self) -> AABB {
        AABB::new(Point3::new(-1., -1., -1.), Point3::new(1., 1., 1.))
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {}
    }
}
