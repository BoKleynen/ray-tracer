#![allow(clippy::many_single_char_names)]

use nalgebra::{Point3, Vector3};

mod accel;
pub mod brdf;
pub mod camera;
pub mod film;
pub mod light;
pub mod material;
pub mod math;
pub mod renderer;
pub mod sampler;
pub mod shade_rec;
pub mod shape;
pub mod world;

pub use accel::bvh;

const K_EPSILON: f64 = 1e-8;

type Float = f64;
pub type Point = Point3<Float>;
pub type Vector = Vector3<Float>;
