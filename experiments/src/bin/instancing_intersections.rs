use cg_practicum::bvh::AxisSelection::Alternate;
use cg_practicum::bvh::SplittingHeuristic::SurfaceAreaHeuristic;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use cg_practicum::light::PointLight;
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point3, Vector};
use experiments::generator::*;
use experiments::{ExperimentResults, SEEDS};
use indicatif::ProgressIterator;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use cg_practicum::camera::CameraBuilder;
use cg_practicum::sampler::Unsampled;
use cg_practicum::renderer::{FalseColorIntersectionTests, Renderer};

const BUNNY_AMOUNTS: [u32; 8] = [1, 2, 5, 10, 15, 20, 30, 40];

fn main() -> Result<(), Box<dyn Error>> {
    let splitting_configs = [SplittingConfig {
        splitting_heuristic: SurfaceAreaHeuristic(12),
        axis_selection: Alternate(Z_AXIS),
    }];

    let camera = CameraBuilder::new(Point3::new(0., 0., 0.))
        .x_res(640)
        .y_res(640)
        .destination(Point3::new(0., 0., -1.))
        .up(Vector::new(0., 1., 0.))
        .fov(90.)
        .build()
        .ok_or("invalid camera configuration")
        .unwrap();
    let sampler = Unsampled::default();
    let tracer = FalseColorIntersectionTests::default();

    let results = splitting_configs
        .iter()
        .map(|&splitting_config| {
            println!("####### Splitting heuristic: {}", splitting_config);

            let experiments = BUNNY_AMOUNTS
                .iter()
                .progress()
                .map(|&nb_bunnies| {
                    SEEDS
                        .iter()
                        .progress()
                        .map(|&seed| {
                            let bunnies = flattened_bunnies_uniform(nb_bunnies, seed);
                            let world = WorldBuilder::default()
                                .light(Box::new(PointLight::white(1., Point3::new(0., 1., 3.))))
                                .geometric_objects(bunnies)
                                .splitting_config(splitting_config)
                                .build()
                                .unwrap();

                            tracer
                                .render_scene(&world, &camera, &sampler)
                                .iter()
                                .sum::<usize>()
                        })
                        .collect()
                })
                .collect();

            (format!("{}", splitting_config), experiments)
        })
        .collect::<HashMap<_, _>>();

    let experiments = ExperimentResults {
        nb_objects: BUNNY_AMOUNTS.iter().copied().collect(),
        results,
    };

    serde_json::to_writer_pretty(
        &File::create("results/flattened_intersections.json")?,
        &experiments,
    )?;

    Ok(())
}
