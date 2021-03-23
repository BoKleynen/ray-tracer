use crate::brdf::Lambertian;
use crate::film::RGB;

#[derive(Debug, Clone, Copy)]
pub struct Emissive {
    // radiance scaling factor, in [0, +inf)
    pub(crate) ls: f64,
    pub(crate) ce: RGB,
}

impl Emissive {
    pub fn new(ls: f64, ce: RGB) -> Self {
        assert!(ls >= 0.);

        Self { ls, ce }
    }
}

#[derive(Debug, Clone)]
pub enum Material {
    // Perfect diffuse reflection
    Matte {
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
    },
    Emissive(Emissive),
}
