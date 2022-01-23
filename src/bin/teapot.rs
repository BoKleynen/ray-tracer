#![allow(unused_imports)]
use ray_tracer::brdf::Lambertian;
use ray_tracer::camera::CameraBuilder;
use ray_tracer::film::Rgb;
use ray_tracer::light::PointLight;
use ray_tracer::material::Material;
use ray_tracer::math::Transformation;
use ray_tracer::renderer::{
    DirectIllumination, FalseColorIntersectionTests, FalseColorNormals, Renderer,
};
use ray_tracer::sampler::JitteredSampler;
use ray_tracer::shape::{GeometricObject, Obj};
use ray_tracer::world::WorldBuilder;
use ray_tracer::{Point3, Vector};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point3::new(0., 2., 4.))
        .x_res(1920)
        .y_res(1080)
        .destination(Point3::new(0., 0.5, 0.))
        .up(Vector::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")?;

    let light1 = PointLight::white(1., Point3::new(2., 4., 5.));
    let t = Transformation::identity();

    let material = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(1., 1., 1.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(1., 1., 1.)),
    };

    let object = Obj::load("models/bunny.obj").unwrap();

    let world = WorldBuilder::default()
        .light(Box::new(light1))
        .background(Rgb::black())
        .geometric_object(GeometricObject::triangle_mesh(object, t, material))
        .build()
        .ok_or("invalid world configuration")?;

    let duration = start.elapsed();
    println!("done building world: {:?}", duration);

    let sampler = JitteredSampler::new(16);
    // let tracer = FalseColorIntersectionTests::default();
    let tracer = DirectIllumination::default();

    let start = Instant::now();

    let buffer = tracer.render_scene(&world, &camera, &sampler);
    // tracer.render_scene(&world, camera, sampler)?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    buffer.to_rgba_image(1., 2.2).save("renders/bunny.png")?;

    Ok(())
}
