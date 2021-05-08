use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::{Camera, CameraBuilder};
use cg_practicum::film::RGB;
use cg_practicum::light::PointLight;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::renderer::{DirectIllumination, Renderer};
use cg_practicum::sampler::Unsampled;
use cg_practicum::shape::{GeometricObject, Obj};
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point, Vector};
use clap::Clap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    let eye = Point::new(-3., 2., 5.);
    let destination = Point::new(5., 0., -1.);
    let up = Vector::new(0., 1., 0.);

    let camera = CameraBuilder::new(eye)
        .x_res(cfg.width)
        .y_res(cfg.height)
        .destination(destination)
        .up(up)
        .fov(cfg.fov)
        .build()
        .ok_or("invalid camera configuration")?;

    // let object = Obj::load("models/teapot.obj").unwrap();

    let material1 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(1., 1., 0.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(1., 1., 0.)),
    };

    let material2 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(1., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(1., 0., 1.)),
    };

    let light = PointLight::white(1., Point::new(100., 50., 150.));
    let light2 = PointLight::white(1., Point::new(50., 100., 50.));

    let world = WorldBuilder::default()
        .geometric_object(GeometricObject::sphere(
            Transformation::translate(1., 1., 0.),
            material1,
        ))
        .geometric_object(GeometricObject::cuboid(
            Point::new(1., 1., 1.),
            Transformation::translate(0., 0., -1.),
            material2.clone(),
        ))
        .light(Box::new(light))
        .light(Box::new(light2))
        .background(RGB::new(0.1, 0.1, 0.1))
        .build()
        .ok_or("invalid world configuration")?;

    let sampler = Unsampled::default();
    let tracer = DirectIllumination::default();
    let buffer = tracer.render_scene(&world, camera, sampler);

    buffer
        .to_rgba_image(cfg.sensitivity, cfg.gamma)
        .save(cfg.filename)?;

    Ok(())
}

#[derive(Clap)]
pub struct Config {
    #[clap(long, default_value = "640")]
    width: usize,
    #[clap(long, default_value = "640")]
    height: usize,
    #[clap(long, default_value = "1")]
    sensitivity: f64,
    #[clap(long, default_value = "2.2")]
    gamma: f64,
    #[clap(long, default_value = "90")]
    fov: f64,
    #[clap(short, long, default_value = "output.png")]
    filename: String,
}
