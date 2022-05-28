mod aabb;
mod interaction;
mod light;
mod material;
mod normal;
mod primitive;
mod shape;
mod transformation;

pub use aabb::{Aabb, Bounded};
pub use interaction::{Shading, SurfaceInteraction};
pub use light::{AreaLight, Light};
pub use material::Material;
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
    pub t_max: Float,
}

impl Default for Ray {
    fn default() -> Self {
        let o = Point3::default();
        let d = Vector3::default();
        let t_max = Float::MAX;

        Self { o, d, t_max }
    }
}
