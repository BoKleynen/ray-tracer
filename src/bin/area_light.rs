#[allow(unused_imports)]
use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::CameraBuilder;
use cg_practicum::film::Rgb;
use cg_practicum::light::{AreaLight, PointLight};
use cg_practicum::material::{Emissive, Material};
use cg_practicum::math::Transformation;
use cg_practicum::renderer::{
    DirectIllumination, FalseColorIntersectionTests, FalseColorNormals, Renderer,
};
use cg_practicum::sampler::{JitteredSampler, RegularSampler, Unsampled};
use cg_practicum::shape::{GeometricObject, Rectangle};
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point, Vector};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point::new(0., 0., 7.))
        .x_res(1920)
        .y_res(1080)
        .destination(Point::new(0., 0., 0.))
        .up(Vector::new(0., 1., 0.))
        .fov(120.)
        .build()
        .ok_or("invalid camera configuration")?;

    let rectangle = Rectangle::new(
        Point::new(1., 5., -1.),
        Vector::new(0., 0., 2.),
        Vector::new(-2., 0., 0.),
    );
    let emissive = Emissive::new(2., Rgb::white());
    let light = AreaLight::new(rectangle, emissive);

    let material1 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(1., 0., 0.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(1., 0., 0.)),
    };
    let sphere = GeometricObject::sphere(Transformation::identity(), material1);

    let material2 = Material::Matte {
        ambient_brdf: Lambertian::new(0., Rgb::new(1., 1., 1.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(1., 1., 1.)),
    };
    let plane = GeometricObject::plane(
        Vector::new(0., 1., 0.),
        Point::new(0., -4., 0.),
        Transformation::identity(),
        material2,
    );

    let world = WorldBuilder::default()
        .background(Rgb::black())
        .light(Box::new(light))
        .geometric_object(sphere)
        .geometric_object(plane)
        .build()
        .ok_or("invalid world configuration")?;

    let sampler = JitteredSampler::new(256);
    let tracer = DirectIllumination::default();
    let buffer = tracer.render_scene(&world, camera, sampler);

    buffer
        .to_rgba_image(1., 2.2)
        .save("renders/area_light.png")?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    Ok(())
}
