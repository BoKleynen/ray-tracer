use cg_practicum::bvh::AxisSelection::Alternate;
use cg_practicum::bvh::SplittingHeuristic::SurfaceAreaHeuristic;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use cg_practicum::camera::CameraBuilder;
use cg_practicum::light::PointLight;
use cg_practicum::renderer::{FalseColorIntersectionTests, Renderer};
use cg_practicum::sampler::Unsampled;
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point3, Vector};
use experiments::scene_generators::generate_uniform_spheres_uniform;
use experiments::{ExperimentResults, SEEDS, SPHERE_AMOUNTS};
use indicatif::ProgressIterator;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

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

            let experiments = SPHERE_AMOUNTS
                .iter()
                .take(5)
                .progress()
                .map(|&nb_spheres| {
                    SEEDS
                        .iter()
                        .progress()
                        .map(|&seed| {
                            let spheres = generate_uniform_spheres_uniform(nb_spheres, seed, 0.025);
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
        nb_spheres: SPHERE_AMOUNTS.iter().copied().collect(),
        results,
    };

    serde_json::to_writer_pretty(
        &File::create("experiments/results/compare_instancing.json")?,
        &experiments,
    )?;

    Ok(())
}
