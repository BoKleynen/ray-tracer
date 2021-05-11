use rayon::prelude::*;

use crate::brdf::Brdf;
use crate::camera::Camera;
use crate::film::{FrameBuffer, Rgb};
use crate::material::Material;
use crate::math::Ray;
use crate::sampler::Sampler;
use crate::shade_rec::ShadeRec;
use crate::world::World;

pub trait Renderer {
    type Output;

    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> Self::Output
    where
        C: Camera + Sync,
        S: Sampler + Sync;
}

#[derive(Default, Debug)]
pub struct DirectIllumination {}

impl Renderer for DirectIllumination {
    type Output = FrameBuffer;

    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> Self::Output
    where
        C: Camera + Sync,
        S: Sampler + Sync,
    {
        let (x_res, y_res) = camera.resolution();
        let mut buffer = FrameBuffer::new(x_res, y_res);

        buffer
            .buffer()
            .par_chunks_exact_mut(x_res)
            .enumerate()
            .for_each(|(r, row)| {
                row.iter_mut().enumerate().for_each(|(c, pixel)| {
                    let color = sampler.average(|sample| {
                        let ray = camera.generate_ray(c, r, sample);

                        match world.hit_objects(&ray) {
                            None => world.background_color(),
                            Some(sr) => Self::shade(&sr.material, &sr, &ray),
                        }
                    });

                    pixel.set(color);
                });
            });

        buffer
    }
}

impl DirectIllumination {
    fn shade(material: &Material, sr: &ShadeRec, ray: &Ray) -> Rgb {
        match material {
            Material::Matte {
                ambient_brdf,
                diffuse_brdf,
            } => {
                let wo = -ray.direction();
                let ambient_radiance =
                    ambient_brdf.rho(sr, &wo) * sr.world.ambient_light().radiance();
                let direct_diffuse_radiance: Rgb = sr
                    .world
                    .lights()
                    .iter()
                    .map(|light| {
                        light.average(&|sample| {
                            let wi = sample.direction(sr);
                            let n_dot_wi = sr.normal.dot(&wi);

                            if n_dot_wi > 0. && sample.visible(&Ray::new(sr.hit_point, *wi), sr) {
                                diffuse_brdf.f(sr, &wo, &wi)
                                    * sample.light().radiance(sr)
                                    * n_dot_wi
                            } else {
                                Rgb::black()
                            }
                        })
                    })
                    .sum();

                ambient_radiance + direct_diffuse_radiance
            }
            Material::Emissive(emissive) => emissive.ce * emissive.ls,
        }
    }
}

#[derive(Default, Debug)]
pub struct FalseColorNormals {}

impl Renderer for FalseColorNormals {
    type Output = FrameBuffer;

    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> Self::Output
    where
        C: Camera + Sync,
        S: Sampler + Sync,
    {
        let (x_res, y_res) = camera.resolution();
        let mut buffer = FrameBuffer::new(x_res, y_res);
        buffer
            .buffer()
            .par_chunks_mut(x_res)
            .enumerate()
            .for_each(|(r, row)| {
                row.iter_mut().enumerate().for_each(|(c, pixel)| {
                    let color = sampler.average(|sample| {
                        let ray = camera.generate_ray(c, r, sample);

                        match world.hit_objects(&ray) {
                            None => world.background_color(),
                            Some(sr) => {
                                Rgb::new(sr.normal.x.abs(), sr.normal.y.abs(), sr.normal.z.abs())
                            }
                        }
                    });

                    pixel.set(color);
                });
            });

        buffer
    }
}

#[derive(Debug)]
pub struct FalseColorIntersectionTests {
    path: String,
}

impl FalseColorIntersectionTests {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Default for FalseColorIntersectionTests {
    fn default() -> Self {
        Self::new("renders/intersection_count.txt".to_owned())
    }
}

impl Renderer for FalseColorIntersectionTests {
    type Output = Vec<usize>;

    // I don't think using sample points makes a lot of sense for this
    fn render_scene<C, S>(&self, world: &World, camera: C, _sampler: S) -> Self::Output
    where
        C: Camera + Sync,
        S: Sampler + Sync,
    {
        let (x_res, y_res) = camera.resolution();
        let mut intersection_counts = vec![0; x_res * y_res];

        intersection_counts
            .par_chunks_mut(x_res)
            .enumerate()
            .for_each(|(r, row)| {
                row.iter_mut().enumerate().for_each(|(c, nb_intersects)| {
                    let ray = camera.generate_ray(c, r, (0.5, 0.5));

                    *nb_intersects = world.count_intersection_tests(&ray)
                })
            });

        intersection_counts
    }
}
