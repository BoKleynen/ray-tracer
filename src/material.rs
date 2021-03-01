use crate::brdf::Lambertian;

#[derive(Debug, Clone)]
pub enum Material {
    // Perfect diffuse reflection
    Matte {
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
    },
}
