use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::CameraBuilder;
use cg_practicum::film::RGB;
use cg_practicum::light::PointLight;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::renderer::{DirectIllumination, Renderer};
use cg_practicum::sampler::Unsampled;
use cg_practicum::shape::{Cuboid, Plane, Sphere};
use cg_practicum::world::WorldBuilder;
use nalgebra::{Point3, Vector3};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point3::new(0., 0., 2.))
        .x_res(640)
        .y_res(640)
        .look_at(Vector3::new(0., 0., -1.))
        .up(Vector3::new(0., 1., 0.))
        .fov(120.)
        .build()
        .ok_or("invalid camera configuration")?;

    let light = PointLight::white(1., Point3::new(0., 0., 0.));

    let white_material = Material::Matte {
        ambient_brdf: Lambertian::new(0., RGB::new(1., 1., 1.)),
        diffuse_brdf: Lambertian::new(0.3, RGB::new(1., 1., 1.)),
    };
    let red_material = Material::Matte {
        ambient_brdf: Lambertian::new(0.05, RGB::new(1., 0., 0.)),
        diffuse_brdf: Lambertian::new(0.3, RGB::new(1., 0., 0.)),
    };
    let green_material = Material::Matte {
        ambient_brdf: Lambertian::new(0.05, RGB::new(0., 1., 0.)),
        diffuse_brdf: Lambertian::new(0.3, RGB::new(0., 1., 0.)),
    };
    let blue_material = Material::Matte {
        ambient_brdf: Lambertian::new(0., RGB::new(0., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.45, RGB::new(0., 0., 1.)),
    };

    let back_plane = Plane::new(
        Vector3::new(0., 0., 1.),
        Point3::new(0., 0., -5.),
        Transformation::identity(),
        white_material.clone(),
    );
    let bottom_plane = Plane::new(
        Vector3::new(0., 1., 0.),
        Point3::new(0., -5., 0.),
        Transformation::identity(),
        white_material.clone(),
    );
    let top_plane = Plane::new(
        Vector3::new(0., -1., 0.),
        Point3::new(0., 5., 0.),
        Transformation::identity(),
        white_material.clone(),
    );
    let left_plane = Plane::new(
        Vector3::new(1., 0., 0.),
        Point3::new(-5., 0., 0.),
        Transformation::identity(),
        red_material,
    );
    let right_plane = Plane::new(
        Vector3::new(-1., 0., 0.),
        Point3::new(5., 0., 0.),
        Transformation::identity(),
        green_material,
    );
    let t1 = Transformation::translate(2., -2., -2.);
    let sphere = Sphere::new(t1, blue_material);

    let world = WorldBuilder::default()
        .background(RGB::black())
        .light(Box::new(light))
        .shape(Box::new(back_plane))
        .shape(Box::new(top_plane))
        .shape(Box::new(bottom_plane))
        .shape(Box::new(left_plane))
        .shape(Box::new(right_plane))
        .shape(Box::new(sphere))
        .build()
        .ok_or("invalid world configuration")?;

    let sampler = Unsampled::default();
    let tracer = DirectIllumination::default();
    let buffer = tracer.render_scene(&world, camera, sampler);

    buffer
        .to_rgba_image(1., 2.2)
        .save("renders/cornell_box.png")?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    Ok(())
}
