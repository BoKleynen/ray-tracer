use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::{Camera, CameraBuilder, ViewPlane};
use cg_practicum::film::RGB;
use cg_practicum::light::PointLight;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::shape::{Cuboid, Obj, Plane, Sphere, TriangleMesh};
use cg_practicum::world::WorldBuilder;
use clap::Clap;
use nalgebra::{Point3, Vector3};
use std::error::Error;
use cg_practicum::sampler::Unsampled;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    let eye = Point3::new(-3., 2., 5.);
    let destination = Point3::new(5., 0., -1.);
    let up = Vector3::new(0., 1., 0.);

    let camera = CameraBuilder::new(eye)
        .x_res(cfg.width)
        .y_res(cfg.height)
        .destination(destination)
        .up(up)
        .fov(cfg.fov)
        .build()
        .ok_or("invalid camera configuration")?;

    let t1 = Transformation::translate(0., 0., -10.).append(&Transformation::scale(5., 5., 5.));
    let t2 = Transformation::translate(4., -4., -12.).append(&Transformation::scale(3., 3., 3.));
    let t3 = Transformation::translate(-4., -4., -12.).append(&Transformation::scale(3., 3., 3.));
    let t4 = Transformation::translate(4., 4., -12.).append(&Transformation::scale(3., 3., 3.));
    let t5 = Transformation::translate(-4., 4., -12.).append(&Transformation::scale(3., 3., 3.));

    // let object = Obj::load("models/teapot.obj").unwrap();

    let material1 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(1., 1., 0.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(1., 1., 0.)),
    };

    let material2 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(1., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(1., 0., 1.)),
    };

    let light = PointLight::white(Point3::new(100., 50., 150.));
    let light2 = PointLight::white(Point3::new(50., 100., 50.));

    let world = WorldBuilder::default()
        // .shape(Box::new(TriangleMesh::new(
        //     object,
        //     material1,
        //     Transformation::identity(),
        // )))
        .shape(Box::new(Sphere::new(
            Transformation::translate(1., 1., 0.),
            material1,
        )))
        .shape(Box::new(Cuboid::new(
            Point3::new(1., 1., 1.),
            Transformation::translate(0., 0., -1.),
            material2.clone(),
        )))
        // .shape(Box::new(Sphere::new(t2, material2.clone())))
        // .shape(Box::new(Sphere::new(t3, green)))
        // .shape(Box::new(Sphere::new(t4, green)))
        // .shape(Box::new(Sphere::new(t5, green)))
        // .add_shape(Box::new(Plane::new(
        //     Vector3::new(1., 1., 0.),
        //     Point3::new(-10., -10., -10.),
        //     Transformation::identity(),
        // )))
        .light(Box::new(light))
        .light(Box::new(light2))
        .background(RGB::new(0.1, 0.1, 0.1))
        .build()
        .ok_or("invalid world configuration")?;

    let vp = ViewPlane {
        horizontal_res: cfg.width,
        vertical_res: cfg.height,
        pixel_size: 0.,
        gamma: 0.,
        inv_gamma: 0.,
    };

    let sampler = Unsampled::new();
    let buffer = camera.render_scene(&world, vp, sampler);

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
