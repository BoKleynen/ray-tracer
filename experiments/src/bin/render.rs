#![allow(unused_imports)]
use cg_practicum::bvh::AxisSelection::{Alternate, Longest};
use cg_practicum::bvh::SplittingHeuristic::SurfaceAreaHeuristic;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use cg_practicum::camera::CameraBuilder;
use cg_practicum::light::PointLight;
use cg_practicum::renderer::{DirectIllumination, FalseColorIntersectionTests, Renderer};
use cg_practicum::sampler::{JitteredSampler, Unsampled};
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point3, Vector};
use experiments::{generator, SEEDS};
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use experiments::generator::*;

fn main() -> Result<(), Box<dyn Error>> {
    let splitting_config = SplittingConfig {
        splitting_heuristic: SurfaceAreaHeuristic(12),
        axis_selection: Longest,
    };
    let camera = CameraBuilder::new(Point3::new(0., 0., 0.))
        .x_res(640)
        .y_res(640)
        .destination(Point3::new(0., 0., -1.))
        .up(Vector::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")
        .unwrap();
    let objects = instanced_bunnies_uniform(200, SEEDS[0]);
    let world = WorldBuilder::default()
        .light(Box::new(PointLight::white(1., Point3::new(0., 1., 3.))))
        .geometric_objects(objects)
        .splitting_config(splitting_config)
        .build()
        .ok_or("invalid world configuration")
        .unwrap();
    let sampler = JitteredSampler::new(16);
    let tracer = DirectIllumination::default();
    let start = Instant::now();
    let buffer = tracer.render_scene(&world, &camera, &sampler);
    let duration = start.elapsed();
    println!("render time: {:?}", duration);

    // File::create("../renders/intersection_tests.txt")?.write_all(
    //     buffer
    //         .iter()
    //         .map(|count| count.to_string())
    //         .join(", ")
    //         .as_bytes(),
    // )?;

    buffer
        .to_rgba_image(1., 2.2)
        .save("../renders/spheres.png")?;

    Ok(())
}
