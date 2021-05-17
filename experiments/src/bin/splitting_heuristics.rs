use cg_practicum::bvh::AxisSelection::*;
use cg_practicum::bvh::SplittingHeuristic::*;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use cg_practicum::camera::CameraBuilder;
use cg_practicum::light::PointLight;
use cg_practicum::renderer::{FalseColorIntersectionTests, Renderer};
use cg_practicum::sampler::Unsampled;
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point3, Vector};
use experiments::generator::*;
use experiments::{ExperimentResults, SEEDS};
use indicatif::ProgressIterator;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub const SPHERE_AMOUNTS: [u32; 15] = [
    100, 500, 1000, 5000, 10_000, 50_000, 100_000, 250_000, 500_000, 750_000, 1_000_000, 2_000_000,
    3_000_000, 4_000_000, 5_000_000,
];

fn main() -> Result<(), Box<dyn Error>> {
    let splitting_configs = [
        SplittingConfig {
            splitting_heuristic: SurfaceAreaHeuristic(12),
            axis_selection: Alternate(Z_AXIS),
        },
        SplittingConfig {
            splitting_heuristic: ObjectMedianSplit,
            axis_selection: Alternate(Z_AXIS),
        },
        SplittingConfig {
            splitting_heuristic: SpaceMedianSplit,
            axis_selection: Alternate(Z_AXIS),
        },
        // SplittingConfig {
        //     splitting_heuristic: SpaceAverageSplit,
        //     axis_selection: Alternate(Z_AXIS),
        // },
        SplittingConfig {
            splitting_heuristic: SurfaceAreaHeuristic(12),
            axis_selection: Longest,
        },
        SplittingConfig {
            splitting_heuristic: ObjectMedianSplit,
            axis_selection: Longest,
        },
        SplittingConfig {
            splitting_heuristic: SpaceMedianSplit,
            axis_selection: Longest,
        },
        // SplittingConfig {
        //     splitting_heuristic: SpaceAverageSplit,
        //     axis_selection: Longest,
        // },
    ];
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

            let experiments = SPHERE_AMOUNTS
                .iter()
                .progress()
                .map(|&nb_spheres| {
                    SEEDS
                        .iter()
                        .progress()
                        .map(|&seed| {
                            let spheres = equal_spheres_normal_yz(nb_spheres, seed, 0.025);
                            let world = WorldBuilder::default()
                                .light(Box::new(PointLight::white(1., Point3::new(0., 1., 3.))))
                                .geometric_objects(spheres)
                                .splitting_config(splitting_config)
                                .build()
                                .ok_or("invalid world configuration")
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
        nb_objects: SPHERE_AMOUNTS.iter().copied().collect(),
        results,
    };

    serde_json::to_writer_pretty(
        &File::create("results/splitting_heuristics_equal_spheres_normal_yz.json")?,
        &experiments,
    )?;

    Ok(())
}
