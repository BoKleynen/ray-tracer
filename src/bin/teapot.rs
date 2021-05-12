#[allow(unused_imports)]
use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::CameraBuilder;
use cg_practicum::film::Rgb;
use cg_practicum::light::PointLight;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::renderer::{
    DirectIllumination, FalseColorIntersectionTests, FalseColorNormals, Renderer,
};
use cg_practicum::sampler::JitteredSampler;
use cg_practicum::shape::{GeometricObject, Obj};
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point3, Vector};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point3::new(0., 0., 0.))
        .x_res(1920)
        .y_res(1080)
        .destination(Point3::new(0., 0., -1.))
        .up(Vector::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")?;

    let light1 = PointLight::white(1., Point3::new(4., -4., 0.));
    let t = Transformation::scale(5., 5., 5.).then(&Transformation::translate(0., -2., -10.));

    let material = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(1., 1., 1.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(1., 1., 1.)),
    };

    let object = Obj::load("models/teapot.obj").unwrap();

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

    let buffer = tracer.render_scene(&world, camera, sampler);
    // tracer.render_scene(&world, camera, sampler)?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    buffer.to_rgba_image(1., 2.2).save("renders/teapot.png")?;

    Ok(())
}
