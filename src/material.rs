use crate::brdf::{Lambertian, BRDF};
use crate::film::RGB;
use crate::math::Ray;
use crate::shade_rec::ShadeRec;

#[derive(Debug, Clone)]
pub enum Material {
    // Perfect diffuse reflection
    Matte {
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
    },
}
