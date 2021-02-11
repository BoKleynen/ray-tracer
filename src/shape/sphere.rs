use crate::math::homogeneous::{Ray, Transformation};
use crate::shape::Shape;

/// Represents a three-dimensional unit sphere, centered at the origin,
/// which is transformed by a transformation.
pub struct Sphere {
    transformation: Transformation,
}

impl Sphere {
    pub fn new(transformation: Transformation) -> Self {
        Sphere { transformation }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let transformed_ray = self.transformation.apply_inverse(ray);

        let origin = transformed_ray.origin().to_vector();
        let direction = transformed_ray.direction().to_vector();

        let a = &direction * &direction;
        let b = &direction * &origin * 2.0;
        let c = &origin * &origin - 1.0;

        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            false
        } else {
            let dr = d.sqrt();

            // numerically solve the equation a*t^2 + b * t + c = 0
            let q = -0.5 * (if b < 0.0 { b - dr } else { b + dr });
            let t0 = q / a;
            let t1 = c / q;

            t0 >= 0.0 || t1 >= 0.0
        }
    }
}
