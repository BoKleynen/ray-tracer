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

impl Material {
    pub fn shade(&self, sr: &ShadeRec, ray: &Ray) -> RGB {
        match self {
            Material::Matte {
                ambient_brdf,
                diffuse_brdf,
            } => {
                let wo = -ray.direction();
                let ambient_radiance =
                    ambient_brdf.rho(sr, &wo) * sr.world.ambient_light().radiance();
                let direct_diffuse_radiance: RGB = sr
                    .world
                    .lights()
                    .iter()
                    .map(|light| {
                        let wi = light.direction(sr);
                        let n_dot_wi = sr.normal.dot(&wi);

                        let shadow_ray = Ray::new(sr.hit_point, wi);
                        let visible =
                            || !sr.world.shapes().iter().any(|shape| shape.hit(&shadow_ray));

                        if n_dot_wi > 0. && visible() {
                            diffuse_brdf.f(sr, &wo, &wi) * light.radiance(sr) * n_dot_wi
                        } else {
                            RGB::black()
                        }
                    })
                    .sum();

                ambient_radiance + direct_diffuse_radiance
            }
        }
    }
}
