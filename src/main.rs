use cg_practicum::camera::{Camera, CameraBuilder, PerspectiveCamera};
use cg_practicum::film::{FrameBuffer, RGB};
use cg_practicum::math::homogeneous::{Point, Transformation, Vector};
use cg_practicum::shape::{Cuboid, Shape, Sphere};
use cg_practicum::world::WorldBuilder;
use clap::Clap;
use rayon::prelude::*;
use std::error::Error;
use std::num::NonZeroUsize;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    let origin = Point::new(0.0, 0.0, 0.0);
    let destination = Point::new(0.0, 0.0, -1.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let camera = CameraBuilder::new(origin)
        .x_res(cfg.width)
        .y_res(cfg.height)
        .destination(destination)
        .up(up.to_vector())
        .fov(cfg.fov)
        .build()
        .ok_or("invalid camera configuration")?;

    let t1 =
        Transformation::translate(0.0, 0.0, -10.0).append(&Transformation::scale(5.0, 5.0, 5.0));
    let t2 =
        Transformation::translate(4.0, -4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));
    let t3 =
        Transformation::translate(-4.0, -4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));
    let t4 =
        Transformation::translate(4.0, 4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));
    let t5 =
        Transformation::translate(-4.0, 4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));

    let world = WorldBuilder::new()
        .camera(camera)
        .add_shape(Box::new(Sphere::new(t1)))
        .add_shape(Box::new(Sphere::new(t2)))
        .add_shape(Box::new(Sphere::new(t3)))
        .add_shape(Box::new(Sphere::new(t4)))
        .add_shape(Box::new(Sphere::new(t5)))
        .build()
        .ok_or("invalid world configuration")?;

    let mut buffer = FrameBuffer::new(cfg.width, cfg.height);
    buffer
        .buffer()
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, pixel)| {
            let x = (idx / cfg.width.get()) as f64;
            let y = (idx % cfg.width.get()) as f64;
            let ray = world.camera().generate_ray((x + 0.5, y + 0.5));
            let hit = world.shapes().iter().any(|shape| shape.intersect(&ray));

            if hit {
                pixel.add(RGB::new(1.0, 0.0, 0.0), 1.0);
            } else {
                pixel.add(RGB::new(0.0, 0.0, 0.0), 1.0);
            }
        });

    buffer
        .to_rgba_image(cfg.sensitivity, cfg.gamma)
        .save(cfg.filename)?;

    Ok(())
}

#[derive(Clap)]
pub struct Config {
    #[clap(long, default_value = "640")]
    width: NonZeroUsize,
    #[clap(long, default_value = "640")]
    height: NonZeroUsize,
    #[clap(long, default_value = "1")]
    sensitivity: f64,
    #[clap(long, default_value = "2.2")]
    gamma: f64,
    #[clap(long, default_value = "90")]
    fov: f64,
    #[clap(short, long, default_value = "output.png")]
    filename: String,
}
