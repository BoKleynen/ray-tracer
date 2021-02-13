use crate::brdf::{Lambertian, BRDF};
use crate::film::RGB;
use crate::math::Ray;
use crate::shade_rec::ShadeRec;

pub enum Material {
    Matte {
        ambient_brdf: Lambertian,
        diffuse_brdf: Lambertian,
    },
}

impl Material {
    pub fn shade(&self, sr: &ShadeRec, ray: &Ray) -> RGB {
        match self {
            Material::Matte {
                ambient_brdf,
                diffuse_brdf,
            } => {
                let wo = -ray.direction();
                let radiance: RGB = sr
                    .world
                    .lights()
                    .iter()
                    .filter_map(|light| {
                        let wi = light.direction(sr);
                        let n_dot_wi = sr.normal.dot(&wi);

                        if n_dot_wi > 0. {
                            let radiance =
                                diffuse_brdf.f(sr, &wo, &wi) * light.radiance(sr) * n_dot_wi;
                            Some(radiance)
                        } else {
                            None
                        }
                    })
                    .sum();

                radiance + ambient_brdf.rho(sr, &wo) * sr.world.ambient_light().radiance()
            }
        }
    }
}
