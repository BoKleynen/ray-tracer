use crate::brdf::{Lambertian, SvLambertian};
use crate::film::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Emissive {
    // radiance scaling factor, in [0, +inf)
    pub(crate) ls: f64,
    pub(crate) ce: Rgb,
}

impl Emissive {
    pub fn new(ls: f64, ce: Rgb) -> Self {
        assert!(ls >= 0.);

        Self { ls, ce }
    }
}

pub enum Material {
    // Perfect diffuse reflection
    Matte {
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
    },
    Emissive(Emissive),
    SvMatte {
        ambient_brdf: SvLambertian,
        diffuse_brdf: SvLambertian,
    },
}
