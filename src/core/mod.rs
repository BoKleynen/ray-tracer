mod aabb;

pub use aabb::Aabb;
use nalgebra::{Point3, Vector3};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const fn all() -> [Self; 3] {
        [Axis::X, Axis::Y, Axis::Z]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}
