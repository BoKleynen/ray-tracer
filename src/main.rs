use cg_practicum::camera::PerspectiveCamera;
use cg_practicum::film::FrameBuffer;
use cg_practicum::math::homogeneous::{Point, Transformation, Vector};
use cg_practicum::shape::{Cuboid, Shape, Sphere};
use clap::Clap;
use rayon::prelude::*;
use std::num::NonZeroUsize;

fn main() {
    let cfg = Config::parse();
    let origin = Point::new(0.0, 0.0, 0.0);
    let destination = Point::new(0.0, 0.0, -1.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let lookat = &destination - &origin;

    let camera = PerspectiveCamera::new(
        cfg.height,
        cfg.width,
        origin,
        lookat.to_vector(),
        up.to_vector(),
        cfg.fov,
    );
    let mut buffer = FrameBuffer::new(cfg.width, cfg.height);

    let t1 =
        Transformation::translate(0.0, 0.0, -10.0).append(&Transformation::scale(5.0, 5.0, 5.0));
    let t2 =
        Transformation::translate(4.0, -4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));
    let t3 =
        Transformation::translate(-4.0, 4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));
    let t4 =
        Transformation::translate(4.0, 4.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));
    let t5 =
        Transformation::translate(-4.0, 0.0, -12.0).append(&Transformation::scale(4.0, 4.0, 4.0));

    let mut shapes: Vec<Box<dyn Shape>> = Vec::with_capacity(5);
    shapes.push(Box::new(Cuboid::new(Point::new(1.0, 1.0, 1.0), t1)));
    shapes.push(Box::new(Sphere::new(t2)));
    shapes.push(Box::new(Sphere::new(t3)));
    shapes.push(Box::new(Sphere::new(t4)));
    shapes.push(Box::new(Sphere::new(t5)));

    // buffer.buffer().par_iter().enumerate().map();
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
