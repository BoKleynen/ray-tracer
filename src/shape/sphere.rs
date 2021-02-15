use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape};
use crate::K_EPSILON;

/// Represents a three-dimensional unit sphere, centered at the origin,
/// which is transformed by a transformation.
pub struct Sphere {
    transformation: Transformation,
    material: Material,
}

impl Sphere {
    pub fn new(transformation: Transformation, material: Material) -> Self {
        Sphere {
            transformation,
            material,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let transformed_ray = self.transformation.apply_inverse(ray);

        let origin = transformed_ray.origin();
        let direction = transformed_ray.direction();

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
            return Some(Hit {
                t,
                normal: (origin + t * direction).coords,
                local_hit_point: origin + t * direction,
            });
        }

        let t = (-b + e) / denom; // larger root
        if t > K_EPSILON {
            return Some(Hit {
                t,
                normal: (origin + t * direction).coords,
                local_hit_point: origin + t * direction,
            });
        }

        None
    }

    fn material(&self) -> Material {
        self.material.clone()
    }
}
