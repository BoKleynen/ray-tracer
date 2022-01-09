pub mod core;

use nalgebra as na;

const K_EPSILON: Float = 1e-8;

pub type Float = f32;

pub type Point2 = na::Point2<Float>;
pub type Point3 = na::Point3<Float>;
pub type Vector3 = na::Vector3<Float>;

#[repr(transparent)]
pub struct Normal3(pub(crate) Vector3);
