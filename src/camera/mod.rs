mod perspective;

pub use perspective::PerspectiveCamera;

use crate::math::homogeneous::Ray;

pub trait Camera {
    fn generate_ray(&self, sample: (f64, f64)) -> Ray;
}
