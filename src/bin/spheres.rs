use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::{Camera, CameraBuilder, ViewPlane};
use cg_practicum::film::RGB;
use cg_practicum::light::PointLight;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::sampler::{RegularSampler, Unsampled};
use cg_practicum::shape::Sphere;
use cg_practicum::world::WorldBuilder;
use nalgebra::{Point3, Vector3};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point3::new(0., 0., 0.))
        .x_res(640)
        .y_res(640)
        .destination(Point3::new(0., 0., -1.))
        // .look_at(Vector3::new(0., 0., -1.))
        .up(Vector3::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")?;

    let light1 = PointLight::white(Point3::new(4., -4., 0.));

    let t1 = Transformation::translate(0., 0., -10.).append(&Transformation::scale(5., 5., 5.));
    let t2 = Transformation::translate(4., -4., -12.).append(&Transformation::scale(4., 4., 3.));
    let t3 = Transformation::translate(-4., -4., -12.).append(&Transformation::scale(4., 4., 3.));
    let t4 = Transformation::translate(4., 4., -12.).append(&Transformation::scale(4., 4., 4.));
    let t5 = Transformation::translate(-4., 4., -12.).append(&Transformation::scale(4., 4., 4.));

    let material1 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(1., 1., 1.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(1., 1., 1.)),
    };
    let material2 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(0., 1., 0.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(0., 1., 0.)),
    };
    let material3 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(0., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(0., 0., 1.)),
    };
    let material4 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(0.5, 0.5, 0.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(0.5, 0.5, 0.)),
    };
    let material5 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, RGB::new(1., 0., 0.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(1., 0., 0.)),
    };

    let world = WorldBuilder::default()
        .light(Box::new(light1))
        .background(RGB::black())
        .shape(Box::new(Sphere::new(t1, material1)))
        .shape(Box::new(Sphere::new(t2, material2)))
        .shape(Box::new(Sphere::new(t3, material3)))
        .shape(Box::new(Sphere::new(t4, material4)))
        .shape(Box::new(Sphere::new(t5, material5)))
        .build()
        .ok_or("invalid world configuration")?;

    let vp = ViewPlane {
        horizontal_res: 640,
        vertical_res: 640,
        pixel_size: 0.,
        gamma: 0.,
        inv_gamma: 0.,
    };
    let sampler = RegularSampler::new(16);
    // let sampler = Unsampled::default();
    let buffer = camera.render_scene(&world, vp, sampler);

    buffer.to_rgba_image(1., 2.2).save("renders/spheres.png")?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    Ok(())
}
