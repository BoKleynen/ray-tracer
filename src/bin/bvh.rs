#[allow(unused_imports)]
use cg_practicum::brdf::Lambertian;
use cg_practicum::camera::CameraBuilder;
use cg_practicum::film::RGB;
use cg_practicum::light::{AreaLight, PointLight};
use cg_practicum::material::{Emissive, Material};
use cg_practicum::math::Transformation;
use cg_practicum::renderer::{
    DirectIllumination, FalseColorIntersectionTests, FalseColorNormals, Renderer,
};
use cg_practicum::sampler::{JitteredSampler, RegularSampler, Unsampled};
use cg_practicum::shape::{Compound, Cuboid, GeometricObject, Rectangle, Shape, Transformed};
use cg_practicum::world::WorldBuilder;
use itertools::Itertools;
use nalgebra::{Point3, Vector3};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    // let total_counts = (1..40).map(|n| {
    //     let camera = CameraBuilder::new(Point3::new(0., 0., 1.))
    //         .x_res(640)
    //         .y_res(640)
    //         .destination(Point3::new(0., 0., 0.))
    //         .up(Vector3::new(0., 1., 0.))
    //         .fov(90.)
    //         .build()
    //         .ok_or("invalid camera configuration").unwrap();
    //
    //     let cubes = generate_cubes_2d(n);
    //
    //     let world = WorldBuilder::default()
    //         .background(RGB::black())
    //         .geometric_object(cubes)
    //         .build()
    //         .ok_or("invalid world configuration").unwrap();
    //
    //     let sampler = Unsampled::default();
    //     let tracer = FalseColorIntersectionTests::default();
    //
    //     tracer.render_scene(&world, camera, sampler).iter().sum::<usize>()
    // })
    //     .collect_vec();

    let camera = CameraBuilder::new(Point3::new(0., 0., 1.))
        .x_res(640)
        .y_res(640)
        .destination(Point3::new(0., 0., 0.))
        .up(Vector3::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")
        .unwrap();

    let cubes = generate_cubes_3d(100);

    let world = WorldBuilder::default()
        .background(RGB::black())
        .geometric_object(cubes)
        .build()
        .ok_or("invalid world configuration")
        .unwrap();

    println!("world build time: {:?}", start.elapsed());

    let start = Instant::now();

    let sampler = Unsampled::default();
    // let tracer = DirectIllumination::default();
    let tracer = FalseColorIntersectionTests::default();

    let res = tracer.render_scene(&world, camera, sampler);

    println!("render time: {:?}", start.elapsed());
    println!("total intersection tests: {}", res.iter().sum::<usize>());

    Ok(())
}

fn generate_cubes_2d(n: usize) -> GeometricObject {
    let material1 = Material::Matte {
        ambient_brdf: Lambertian::new(0.65, RGB::new(0., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(0., 0., 1.)),
    };
    let cuboid = Cuboid::new(Point3::new(0.5, 0.5, 0.5));
    let m = n as f64;
    let inv_m = 1. / m;

    let cuboids = (0..n)
        .cartesian_product((0..n))
        .map(|(p, q)| {
            let x = (-0.5 + inv_m / 2.) + p as f64 * inv_m;
            let y = (-0.5 + inv_m / 2.) + q as f64 * inv_m;
            let z = 0.;
            let transformation =
                Transformation::scale(inv_m, inv_m, 1.).then(&Transformation::translate(x, y, z));
            Transformed::new(cuboid.clone(), transformation)
        })
        .collect_vec();

    GeometricObject::new(Box::new(Compound::new(cuboids)), material1)
}

fn generate_cubes_3d(n: usize) -> GeometricObject {
    let material1 = Material::Matte {
        ambient_brdf: Lambertian::new(0.65, RGB::new(0., 0., 1.)),
        diffuse_brdf: Lambertian::new(0.65, RGB::new(0., 0., 1.)),
    };
    let cuboid = Cuboid::new(Point3::new(0.5, 0.5, 0.5));
    let m = n as f64;
    let inv_m = 1. / m;

    let cuboids = (0..n)
        .cartesian_product((0..n))
        .cartesian_product((0..n))
        .map(|((p, q), r)| {
            let x = (-0.5 + inv_m / 2.) + p as f64 * inv_m;
            let y = (-0.5 + inv_m / 2.) + q as f64 * inv_m;
            let z = (-0.5 + inv_m / 2.) + r as f64 * inv_m;
            let transformation = Transformation::scale(inv_m, inv_m, inv_m)
                .then(&Transformation::translate(x, y, z));
            Transformed::new(cuboid.clone(), transformation)
        })
        .collect_vec();

    GeometricObject::new(Box::new(Compound::new(cuboids)), material1)
}
