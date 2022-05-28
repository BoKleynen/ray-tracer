pub mod core;
pub mod shape;

pub(crate) mod util;

#[cfg(not(feature = "float_as_double"))]
pub type Float = f32;
#[cfg(feature = "float_as_double")]
pub type Float = f64;
