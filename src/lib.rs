#![allow(clippy::many_single_char_names)]

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
pub mod texture;
pub mod world;

pub use accel::bvh;

const K_EPSILON: f64 = 1e-8;

pub type Float = f64;
pub type Point2 = nalgebra::Point2<Float>;
pub type Point3 = nalgebra::Point3<Float>;
pub type Vector = nalgebra::Vector3<Float>;
