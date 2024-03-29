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
use ray_tracer::sampler::{JitteredSampler, RegularSampler, Unsampled};
use ray_tracer::shape::GeometricObject;
use ray_tracer::world::WorldBuilder;
use ray_tracer::{Point3, Vector};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let camera = CameraBuilder::new(Point3::new(0., 0., 0.))
        .x_res(1920)
        .y_res(1080)
        .destination(Point3::new(0., 0., -1.))
        // .look_at(Vector::new(0., 0., -1.))
        .up(Vector::new(0., 1., 0.))
        .fov(120.)
        .build()
        .ok_or("invalid camera configuration")?;

    let light1 = PointLight::white(1., Point3::new(4., -4., 0.));

    let t1 = Transformation::scale(5., 5., 5.).then(&Transformation::translate(0., 0., -10.));
    let t2 = Transformation::scale(4., 4., 3.).then(&Transformation::translate(4., -4., -12.));
    let t3 = Transformation::scale(4., 4., 3.).then(&Transformation::translate(-4., -4., -12.));
    let t4 = Transformation::scale(4., 4., 4.).then(&Transformation::translate(4., 4., -12.));
    let t5 = Transformation::scale(4., 4., 4.).then(&Transformation::translate(-4., 4., -12.));

    let material1 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(1., 1., 1.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(1., 1., 1.)),
    };
    let material2 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(0., 1., 0.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(0., 1., 0.)),
    };
    let material3 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(0., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(0., 0., 1.)),
    };
    let material4 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(0.5, 0.5, 0.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(0.5, 0.5, 0.)),
    };
    let material5 = Material::Matte {
        ambient_brdf: Lambertian::new(0.15, Rgb::new(1., 0., 0.)),
        diffuse_brdf: Lambertian::new(0.65, Rgb::new(1., 0., 0.)),
    };

    let world = WorldBuilder::default()
        .light(Box::new(light1))
        .background(Rgb::black())
        .geometric_object(GeometricObject::sphere(t1, material1))
        .geometric_object(GeometricObject::sphere(t2, material2))
        .geometric_object(GeometricObject::sphere(t3, material3))
        .geometric_object(GeometricObject::sphere(t4, material4))
        .geometric_object(GeometricObject::sphere(t5, material5))
        .build()
        .ok_or("invalid world configuration")?;

    // let sampler = RegularSampler::new(16);
    // let sampler = Unsampled::default();
    let sampler = JitteredSampler::new(16);
    let tracer = DirectIllumination::default();
    let buffer = tracer.render_scene(&world, &camera, &sampler);

    buffer.to_rgba_image(1., 2.2).save("renders/spheres.png")?;

    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    Ok(())
}
