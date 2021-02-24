use crate::camera::Camera;
use crate::film::{FrameBuffer, RGB};
use crate::sampler::Sampler;
use crate::world::World;
use rayon::prelude::*;

pub trait Tracer {
    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> FrameBuffer
    where
        C: Camera + Sync,
        S: Sampler + Sync;
}

#[derive(Default, Debug)]
pub struct Renderer {}

impl Tracer for Renderer {
    fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> FrameBuffer
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
                            Some(sr) => sr.material.shade(&sr, &ray),
                        }
                    });

                    pixel.set(color);
                });
            });

        buffer
    }
}

#[derive(Default, Debug)]
pub struct FalseColorNormal {}

impl Tracer for FalseColorNormal {
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

// #[derive(Default,Debug)]
// pub struct FalseColorIntersectionTests {}
//
// impl Tracer for FalseColorIntersectionTests {
//     fn render_scene<C, S>(&self, world: &World, camera: C, sampler: S) -> FrameBuffer
//     where
//         C: Camera + Sync,
//         S: Sampler + Sync
//     {
//         let (x_res, y_res) = camera.resolution();
//         let intersection_counts = vec![0; x_res * y_res];
//
//         intersection_counts.par_chunks_mut(x_res)
//             .enumerate()
//             .for_each(|(r, row)| {
//                 row.iter_mut().enumerate().for_each(|(c, nb_intersects)| {
//                     *nb_intersects = 1
//                 })
//             });
//     }
// }
