use cg_practicum::bvh::AxisSelection::*;
use cg_practicum::bvh::SplittingHeuristic::*;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use cg_practicum::light::PointLight;
use cg_practicum::shape::GeometricObject;
use cg_practicum::world::WorldBuilder;
use cg_practicum::Point3;
use experiments::generator::*;
use experiments::{ExperimentResults, SEEDS};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

const SPHERE_AMOUNTS: [u32; 15] = [
    100, 500, 1000, 5000, 10_000, 50_000, 100_000, 250_000, 500_000, 750_000, 1_000_000, 2_000_000,
    3_000_000, 4_000_000, 5_000_000,
];

const GENERATORS: [(&str, GeneratorFn); 6] = [
    ("equal_spheres_uniform", equal_spheres_uniform),
    (
        "uniform_spheres_uniform_position",
        uniform_spheres_uniform_position,
    ),
    ("equal_spheres_uniform_yz", equal_spheres_uniform_yz),
    ("equal_spheres_normal_yz", equal_spheres_normal_yz),
    ("equal_spheres_beta_corners", equal_spheres_beta_corners),
    ("equal_spheres_beta_x", equal_spheres_beta_x),
];

type GeneratorFn = fn(u32, <ChaCha8Rng as SeedableRng>::Seed, f64) -> Vec<GeometricObject>;

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
    ];

    GENERATORS.iter().for_each(|(description, generator)| {
        let results = splitting_configs
            .iter()
            .map(|&splitting_config| {
                let experiments = SPHERE_AMOUNTS
                    .iter()
                    .map(|&nb_spheres| {
                        SEEDS
                            .iter()
                            .flat_map(|&seed| {
                                (0..10).map(move |_| {
                                    let spheres = generator(nb_spheres, seed, 0.025);
                                    let start = Instant::now();
                                    WorldBuilder::default()
                                        .light(Box::new(PointLight::white(
                                            1.,
                                            Point3::new(0., 1., 3.),
                                        )))
                                        .geometric_objects(spheres)
                                        .splitting_config(splitting_config)
                                        .build()
                                        .ok_or("invalid world configuration")
                                        .unwrap();

                                    start.elapsed()
                                })
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
            &File::create(format!(
                "results/splitting_heuristics_{}_time.json",
                description
            ))
            .unwrap(),
            &experiments,
        )
        .unwrap();
    });

    Ok(())
}
