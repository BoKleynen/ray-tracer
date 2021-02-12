use crate::film::RGB;
use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape};
use nalgebra::{Point3, Vector3};

/// Represents a three-dimensional unit sphere, centered at the origin,
/// which is transformed by a transformation.
pub struct Sphere {
    transformation: Transformation,
    color: RGB,
}

impl Sphere {
    pub fn new(transformation: Transformation, color: RGB) -> Self {
        Sphere {
            transformation,
            color,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let transformed_ray = self.transformation.apply_inverse(ray);

        let origin = transformed_ray.origin();
        let direction = transformed_ray.direction();

        let a = direction.norm_squared();
        let b = 2.0 * &origin.coords.dot(direction);
        let c = origin.coords.dot(&origin.coords) - 1.0;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let e = disc.sqrt();
        let denom = 2.0 * a;

        let t = (-b - e) / denom; // smaller root
        if t > f64::EPSILON {
            return Some(Hit {
                t,
                normal: &origin.coords + t * direction,
                local_hit_point: origin + t * direction,
            });
        }

        let t = (-b + e) / denom;
        if t > f64::EPSILON {
            return Some(Hit {
                t,
                normal: &origin.coords + t * direction,
                local_hit_point: origin + t * direction,
            });
        }

        None
    }

    fn color(&self) -> RGB {
        self.color
    }
}
