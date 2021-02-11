use cg_practicum::camera::{PerspectiveCamera, Camera};
use cg_practicum::film::{FrameBuffer, RGB};
use cg_practicum::math::homogeneous::{Point, Transformation, Vector};
use cg_practicum::shape::{Cuboid, Shape, Sphere};
use clap::Clap;
use rayon::prelude::*;
use std::error::Error;
use std::num::NonZeroUsize;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    let origin = Point::new(0.0, 0.0, 0.0);
    let destination = Point::new(0.0, 0.0, -1.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let lookat = &destination - &origin;

    let camera = PerspectiveCamera::new(
        cfg.width,
        cfg.height,
        origin,
        lookat.to_vector(),
        up.to_vector(),
        cfg.fov,
    );
    let mut buffer = FrameBuffer::new(cfg.width, cfg.height);

    let t1 =
        Transformation::translate(0.0, 0.0, -10.0).append(&Transformation::scale(50.0, 50.0, 50.0));
    let t2 =
        Transformation::translate(4.0, -4.0, -12.0).append(&Transformation::scale(40.0, 40.0, 40.0));
    let t3 =
        Transformation::translate(-4.0, -4.0, -12.0).append(&Transformation::scale(40.0, 40.0, 40.0));
    let t4 =
        Transformation::translate(4.0, 4.0, -12.0).append(&Transformation::scale(40.0, 40.0, 40.0));
    let t5 =
        Transformation::translate(-4.0, 4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));

    let mut shapes: Vec<Box<dyn Shape + Sync + Send>> = Vec::with_capacity(5);
    // shapes.push(Box::new(Cuboid::new(Point::new(1.0, 1.0, 1.0), t1)));
    // shapes.push(Box::new(Sphere::new(t1)));
    shapes.push(Box::new(Sphere::new(t2)));
    shapes.push(Box::new(Sphere::new(t3)));
    shapes.push(Box::new(Sphere::new(t4)));
    shapes.push(Box::new(Sphere::new(t5)));

    buffer.buffer().par_iter_mut().enumerate().for_each(|(idx, pixel)| {
        let x = (idx / cfg.width.get()) as f64;
        let y = (idx % cfg.width.get()) as f64;

        let ray = camera.generate_ray((x+0.5, y+0.5));

        let hit = shapes.iter().any(|shape| shape.intersect(&ray));

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
