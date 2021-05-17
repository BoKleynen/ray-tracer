use cg_practicum::bvh::AxisSelection::Alternate;
use cg_practicum::bvh::SplittingHeuristic::SurfaceAreaHeuristic;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use cg_practicum::light::PointLight;
use cg_practicum::world::WorldBuilder;
use cg_practicum::Point3;
use experiments::generator::*;
use experiments::{ExperimentResults, SEEDS};
use indicatif::ProgressIterator;
use jemalloc_ctl::{epoch, stats};
use jemallocator::Jemalloc;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use cg_practicum::shape::GeometricObject;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

const BUNNY_AMOUNTS: [u32; 8] = [1, 2, 5, 10, 15, 20, 30, 40];

fn main() -> Result<(), Box<dyn Error>> {
    let e = epoch::mib().unwrap();
    let allocated = stats::allocated::mib().unwrap();

    let splitting_configs = [SplittingConfig {
        splitting_heuristic: SurfaceAreaHeuristic(12),
        axis_selection: Alternate(Z_AXIS),
    }];

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
                            e.advance().unwrap();
                            let start = allocated.read().unwrap();

                            let bunnies = flattened_bunnies_uniform(nb_bunnies, seed);
                            let _world = WorldBuilder::default()
                                .light(Box::new(PointLight::white(1., Point3::new(0., 1., 3.))))
                                .geometric_objects(bunnies)
                                .splitting_config(splitting_config)
                                .build()
                                .unwrap();

                            e.advance().unwrap();
                            let end = allocated.read().unwrap();

                            end - start
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
        &File::create("results/flattened_memory.json")?,
        &experiments,
    )?;

    Ok(())
}
