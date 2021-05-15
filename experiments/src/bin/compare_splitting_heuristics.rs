use cg_practicum::brdf::Lambertian;
use cg_practicum::bvh::AxisSelection::*;
use cg_practicum::bvh::SplittingHeuristic::*;
use cg_practicum::bvh::{SplittingConfig, Z_AXIS};
use cg_practicum::camera::CameraBuilder;
use cg_practicum::film::Rgb;
use cg_practicum::light::PointLight;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::renderer::{FalseColorIntersectionTests, Renderer};
use cg_practicum::sampler::Unsampled;
use cg_practicum::shape::GeometricObject;
use cg_practicum::world::WorldBuilder;
use cg_practicum::{Point3, Vector};
use experiments::{ExperimentResults, SEEDS, SPHERE_AMOUNTS};
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_distr::Uniform;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::f64::consts::FRAC_1_PI;
use std::fs::File;

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
            splitting_heuristic: SpaceAverageSplit,
            axis_selection: Alternate(Z_AXIS),
        },
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
                .take(5)
                .progress()
                .map(|&nb_spheres| {
                    SEEDS
                        .iter()
                        .progress()
                        .map(|&seed| {
                            let spheres = generate_uniform_spheres_uniform(nb_spheres, seed, 0.025);
                            let world = WorldBuilder::default()
                                .background(Rgb::black())
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
        &File::create("experiments/results/compare_splitting_heuristics3.json")?,
        &experiments,
    )?;

    Ok(())
}

fn generate_equal_spheres_uniform(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    let radius = (0.75 * FRAC_1_PI * fill / nb_spheres as f64).powf(1. / 3.);

    let mut rng = ChaCha8Rng::from_seed(seed);
    let position_distribution = Uniform::new_inclusive(-0.5, 0.5);
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_spheres)
        .map(|_| {
            let transformation =
                Transformation::scale(radius, radius, radius).then(&Transformation::translate(
                    rng.sample(position_distribution),
                    rng.sample(position_distribution),
                    rng.sample(position_distribution) - 1.,
                ));
            let color = Rgb::new(
                rng.sample(color_distribution),
                rng.sample(color_distribution),
                rng.sample(color_distribution),
            );
            let material = Material::Matte {
                ambient_brdf: Lambertian::new(0.15, color),
                diffuse_brdf: Lambertian::new(0.75, color),
            };

            GeometricObject::sphere(transformation, material)
        })
        .collect_vec()
}

fn generate_uniform_spheres_uniform(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    let min_radius = (1. / 52. * FRAC_1_PI * fill / nb_spheres as f64).powf(1. / 3.);

    let mut rng = ChaCha8Rng::from_seed(seed);
    let radius_distribution = Uniform::new(min_radius, 5. * min_radius);
    let position_distribution = Uniform::new_inclusive(-0.5, 0.5);
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_spheres)
        .map(|_| {
            let radius = rng.sample(radius_distribution);
            let transformation =
                Transformation::scale(radius, radius, radius).then(&Transformation::translate(
                    rng.sample(position_distribution),
                    rng.sample(position_distribution),
                    rng.sample(position_distribution) - 1.,
                ));
            let color = Rgb::new(
                rng.sample(color_distribution),
                rng.sample(color_distribution),
                rng.sample(color_distribution),
            );
            let material = Material::Matte {
                ambient_brdf: Lambertian::new(0.15, color),
                diffuse_brdf: Lambertian::new(0.75, color),
            };

            GeometricObject::sphere(transformation, material)
        })
        .collect_vec()
}

fn generate_spheres_skewed(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    todo!()
}
