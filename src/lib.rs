#![allow(clippy::many_single_char_names)]

pub mod brdf;
#[cfg(feature = "bvh")]
pub mod bvh;
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

const K_EPSILON: f64 = 1e-10;
