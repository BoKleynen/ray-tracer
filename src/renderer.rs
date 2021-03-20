use crate::brdf::BRDF;
use crate::bvh::ShapeNode;
use crate::camera::Camera;
use crate::film::{FrameBuffer, RGB};
use crate::material::Material;
use crate::math::Ray;
use crate::sampler::Sampler;
use crate::shade_rec::ShadeRec;
use crate::world::World;
use itertools::Itertools;
use nalgebra::Vector3;
use rayon::prelude::*;

pub trait Renderer {
    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> FrameBuffer
    where
        C: Camera + Sync,
        S: Sampler + Sync;
}

#[derive(Default, Debug)]
pub struct DirectIllumination {}

fn hit_objects<'a>(world: &'a World, nodes: &[ShapeNode], ray: &Ray) -> Option<ShadeRec<'a>> {
    let mut sr: Option<ShadeRec> = None;
    let mut t_min = f64::INFINITY;

    nodes.iter().for_each(|ShapeNode { aabb, obj }| {
        if aabb.intersect(ray).is_some() {
            if let Some(hit) = obj.intersect(&ray) {
                if hit.t < t_min {
                    t_min = hit.t;

                    sr = Some(ShadeRec {
                        hit_point: ray.origin() + hit.t * ray.direction(),
                        local_hit_point: hit.local_hit_point,
                        normal: hit.normal,
                        material: obj.material(),
                        depth: 0,
                        direction: Vector3::default(),
                        world,
                    })
                }
            }
        }
    });

    sr
}

impl Renderer for DirectIllumination {
    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> FrameBuffer
    where
        C: Camera + Sync,
        S: Sampler + Sync,
    {
        let (x_res, y_res) = camera.resolution();
        let mut buffer = FrameBuffer::new(x_res, y_res);

        let nodes = world
            .geometric_objects()
            .iter()
            .map(|obj| {
                let aabb = obj.shape().bbox();
                ShapeNode { aabb, obj }
            })
            .collect_vec();

        buffer
            .buffer()
            .par_chunks_exact_mut(x_res)
            .enumerate()
            .for_each(|(r, row)| {
                row.iter_mut().enumerate().for_each(|(c, pixel)| {
                    let color = sampler.average(|sample| {
                        let ray = camera.generate_ray(c, r, sample);

                        match hit_objects(world, &nodes, &ray) {
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
    fn shade(material: &Material, sr: &ShadeRec, ray: &Ray) -> RGB {
        match material {
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

                        if n_dot_wi > 0. && light.visible(&Ray::new(sr.hit_point, *wi), sr) {
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

#[derive(Default, Debug)]
pub struct FalseColorNormals {}

impl Renderer for FalseColorNormals {
    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> FrameBuffer
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
                            Some(sr) => RGB::new(sr.normal.x, sr.normal.y, sr.normal.z),
                        }
                    });

                    pixel.set(color);
                });
            });

        buffer
    }
}

#[derive(Default, Debug)]
pub struct FalseColorIntersectionTests {}

impl Renderer for FalseColorIntersectionTests {
    // I don't think using sample points makes a lot of sense for this
    fn render_scene<C, S>(&self, world: &World, camera: C, _sampler: S) -> FrameBuffer
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

                    *nb_intersects = world
                        .geometric_objects()
                        .iter()
                        .map(|shape| shape.count_intersection_tests(&ray))
                        .sum();
                })
            });

        let mut buffer = FrameBuffer::new(x_res, y_res);
        buffer
            .buffer()
            .iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let count = intersection_counts[idx];

                let r = (count & 0b1111_1111 << 16) >> 16;
                let g = (count & 0b1111_1111 << 8) >> 8;
                let b = count & 0b_1111_1111;

                pixel.set(RGB::new(r as f64, g as f64, b as f64));
            });

        buffer
    }
}
