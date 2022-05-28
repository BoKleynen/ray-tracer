mod aabb;
mod transformation;

pub use aabb::{Aabb, Union};
pub use interaction::SurfaceInteraction;
pub use transformation::{Transformable, Transformation};

use crate::Float;
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
    pub origin: Point3<Float>,
    pub direction: Vector3<Float>,
}
