use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::CameraBuilder;
use cg_practicum::film::RGB;
use cg_practicum::light::PointLight;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::renderer::{DirectIllumination, Renderer};
use cg_practicum::sampler::{JitteredSampler, Unsampled};
use cg_practicum::shape::GeometricObject;
use cg_practicum::world::WorldBuilder;
use nalgebra::{Point3, Vector3};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point3::new(0., 0., 4.))
        .x_res(1080)
        .y_res(1080)
        .look_at(Vector3::new(0., 0., -1.))
        .up(Vector3::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")?;

    let light = PointLight::white(1., Point3::new(0., 2., -1.));

    let white_material = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(238. / 255., 235. / 255., 227. / 255.)),
        diffuse_brdf: Lambertian::new(0.5, RGB::new(1., 1., 1.)),
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
        ambient_brdf: Lambertian::new(0.15, RGB::new(0., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.45, RGB::new(0., 0., 1.)),
    };

    let back_plane = GeometricObject::plane(
        Vector3::new(0., 0., 1.),
        Point3::new(0., 0., -5.),
        Transformation::identity(),
        white_material.clone(),
    );
    let bottom_plane = GeometricObject::plane(
        Vector3::new(0., 1., 0.),
        Point3::new(0., -5., 0.),
        Transformation::identity(),
        white_material.clone(),
    );
    let top_plane = GeometricObject::plane(
        Vector3::new(0., -1., 0.),
        Point3::new(0., 5., 0.),
        Transformation::identity(),
        white_material.clone(),
    );
    let left_plane = GeometricObject::plane(
        Vector3::new(1., 0., 0.),
        Point3::new(-5., 0., 0.),
        Transformation::identity(),
        red_material,
    );
    let right_plane = GeometricObject::plane(
        Vector3::new(-1., 0., 0.),
        Point3::new(5., 0., 0.),
        Transformation::identity(),
        green_material,
    );

    let t2 = Transformation::rotate_y(-40.).then(&Transformation::translate(1.75, -3.5, -2.5));
    let cube = GeometricObject::cuboid(Point3::new(1.5, 1.5, 1.5), t2, blue_material.clone());

    let t3 = Transformation::rotate_y(35.).then(&Transformation::translate(-2.5, -3., -4.));
    let cuboid = GeometricObject::cuboid(Point3::new(1.25, 3.5, 1.25), t3, white_material.clone());

    let world = WorldBuilder::default()
        .background(RGB::black())
        .light(Box::new(light))
        .geometric_object(back_plane)
        .geometric_object(top_plane)
        .geometric_object(bottom_plane)
        .geometric_object(left_plane)
        .geometric_object(right_plane)
        .geometric_object(cube)
        .geometric_object(cuboid)
        .build()
        .ok_or("invalid world configuration")?;

    let sampler = JitteredSampler::new(9);
    let tracer = DirectIllumination::default();
    let buffer = tracer.render_scene(&world, camera, sampler);

    buffer
        .to_rgba_image(1., 2.2)
        .save("renders/cornell_box.png")?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    Ok(())
}
