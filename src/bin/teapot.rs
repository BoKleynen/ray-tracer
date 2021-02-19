use cg_practicum::camera::{CameraBuilder, ViewPlane, Camera};
use nalgebra::{Point3, Vector3};
use cg_practicum::light::PointLight;
use cg_practicum::math::Transformation;
use cg_practicum::material::Material;
use cg_practicum::brdf::Lambertian;
use cg_practicum::film::RGB;
use cg_practicum::world::WorldBuilder;
use cg_practicum::shape::{Sphere, Obj, TriangleMesh};
use cg_practicum::sampler::JitteredSampler;
use std::time::Instant;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point3::new(0., 0., 0.))
        .x_res(1920)
        .y_res(1080)
        .destination(Point3::new(0., 0., -1.))
        // .look_at(Vector3::new(0., 0., -1.))
        .up(Vector3::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")?;

    let light1 = PointLight::white(Point3::new(4., -4., 0.));
    let t = Transformation::translate(0., 0., -10.).append(&Transformation::scale(5., 5., 5.));

    let material = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(1., 1., 1.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(1., 1., 1.)),
    };

    let object = Obj::load("models/teapot.obj").unwrap();


    let world = WorldBuilder::default()
        .light(Box::new(light1))
        .background(RGB::black())
        .shape(Box::new(TriangleMesh::new(
            object,
            material,
            t,
        )))
        .build()
        .ok_or("invalid world configuration")?;

    let vp = ViewPlane {
        horizontal_res: 1920,
        vertical_res: 1080,
        pixel_size: 0.,
        gamma: 0.,
        inv_gamma: 0.,
    };

    let sampler = JitteredSampler::new(16);
    let buffer = camera.render_scene(&world, vp, sampler);

    buffer.to_rgba_image(1., 2.2).save("renders/teapot.png")?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    Ok(())
}