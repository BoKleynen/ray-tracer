use cg_practicum::camera::CameraBuilder;
use cg_practicum::film::RGB;
use cg_practicum::math::Transformation;
use cg_practicum::shape::{Cuboid, Obj, Plane, Sphere, TriangleMesh};
use cg_practicum::world::WorldBuilder;
use clap::Clap;
use nalgebra::{Point3, Vector3};
use std::error::Error;
use std::num::NonZeroUsize;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    let origin = Point3::new(0.0, 0.0, 0.0);
    let destination = Point3::new(0.0, 0.0, -1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);

    let camera = CameraBuilder::new(origin)
        .x_res(cfg.width)
        .y_res(cfg.height)
        .destination(destination)
        .up(up)
        .fov(cfg.fov)
        .build()
        .ok_or("invalid camera configuration")?;

    let t1 =
        Transformation::translate(0.0, 0.0, -10.0).append(&Transformation::scale(5.0, 5.0, 5.0));
    let t2 =
        Transformation::translate(4.0, -4.0, -12.0).append(&Transformation::scale(3.0, 3.0, 3.0));
    let t3 =
        Transformation::translate(-4.0, -4.0, -12.0).append(&Transformation::scale(3.0, 3.0, 3.0));
    let t4 =
        Transformation::translate(4.0, 4.0, -12.0).append(&Transformation::scale(3.0, 3.0, 3.0));
    let t5 =
        Transformation::translate(-4.0, 4.0, -12.0).append(&Transformation::scale(3.0, 3.0, 3.0));

    // let object = Obj::load("models/teapot.obj").unwrap();

    let green = RGB::new(0.0, 1.0, 0.0);
    let red = RGB::new(1.0, 0.0, 0.0);

    let world = WorldBuilder::new()
        .camera(camera)
        // .add_shape(Box::new(TriangleMesh::new(object, t1)))
        .add_shape(Box::new(Cuboid::new(Point3::new(0.5, 0.5, 0.5), t1, red)))
        // .add_shape(Box::new(Sphere::new(t1, red)))
        .add_shape(Box::new(Sphere::new(t2, green)))
        .add_shape(Box::new(Sphere::new(t3, green)))
        .add_shape(Box::new(Sphere::new(t4, green)))
        .add_shape(Box::new(Sphere::new(t5, green)))
        // .add_shape(Box::new(Plane::new(
        //     Vector3::new(1.0, 1.0, 0.0),
        //     Point3::new(-10.0, -10.0, -10.0),
        //     Transformation::identity(),
        // )))
        .build()
        .ok_or("invalid world configuration")?;

    let buffer = world.render_scene(cfg.width, cfg.height)?;

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
