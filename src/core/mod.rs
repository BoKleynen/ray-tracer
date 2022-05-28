mod aabb;
mod interaction;
mod normal;
mod shape;
mod transformation;

pub use aabb::{Aabb, Union};
pub use interaction::SurfaceInteraction;
pub use normal::Normal3;
pub use shape::Shape;
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
    /// The origin of the ray.
    pub o: Point3<Float>,
    /// The direction of the ray.
    pub d: Vector3<Float>,
}

impl Default for Ray {
    fn default() -> Self {
        let o = Point3::default();
        let d = Vector3::default();

        Self { o, d }
    }
}
