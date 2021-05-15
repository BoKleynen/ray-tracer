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

const SEEDS: [[u8; 32]; 10] = [
    [
        166, 42, 82, 165, 155, 189, 65, 184, 126, 128, 77, 169, 48, 171, 71, 38, 52, 217, 207, 164,
        190, 4, 184, 153, 229, 219, 172, 163, 65, 231, 20, 226,
    ],
    [
        103, 90, 45, 175, 7, 56, 49, 57, 150, 147, 158, 144, 169, 36, 234, 215, 75, 211, 48, 51,
        137, 188, 213, 10, 183, 244, 183, 150, 56, 30, 124, 50,
    ],
    [
        47, 7, 124, 250, 21, 227, 10, 182, 41, 254, 142, 250, 191, 6, 228, 34, 130, 242, 220, 48,
        31, 146, 170, 11, 107, 104, 221, 175, 31, 169, 75, 115,
    ],
    [
        117, 146, 64, 150, 47, 125, 63, 107, 150, 155, 191, 168, 121, 238, 23, 185, 130, 139, 174,
        23, 54, 21, 128, 97, 187, 9, 218, 161, 248, 75, 133, 124,
    ],
    [
        8, 194, 206, 54, 227, 73, 140, 33, 160, 10, 215, 211, 205, 130, 166, 25, 131, 52, 10, 15,
        254, 110, 191, 215, 10, 52, 191, 4, 14, 47, 231, 46,
    ],
    [
        66, 192, 119, 57, 30, 6, 196, 74, 240, 224, 252, 207, 221, 48, 226, 220, 101, 152, 3, 244,
        0, 237, 86, 179, 117, 73, 161, 210, 228, 189, 194, 234,
    ],
    [
        97, 181, 186, 204, 227, 187, 249, 192, 165, 140, 0, 128, 123, 33, 73, 137, 128, 46, 231,
        57, 59, 204, 87, 133, 12, 139, 134, 222, 62, 47, 123, 16,
    ],
    [
        20, 97, 149, 164, 147, 86, 222, 220, 100, 14, 52, 252, 82, 87, 93, 245, 222, 13, 182, 76,
        26, 189, 216, 134, 9, 62, 59, 239, 118, 19, 189, 28,
    ],
    [
        179, 22, 98, 82, 157, 48, 24, 78, 57, 52, 228, 150, 104, 71, 241, 76, 15, 27, 213, 240, 92,
        81, 121, 119, 7, 145, 40, 161, 141, 187, 227, 169,
    ],
    [
        59, 102, 60, 110, 112, 14, 241, 138, 5, 114, 1, 194, 101, 5, 119, 93, 208, 109, 253, 70,
        25, 150, 178, 220, 152, 98, 221, 31, 132, 20, 83, 220,
    ],
];
const SPHERE_AMOUNTS: [u32; 15] = [
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
                            let spheres = generate_uniform_spheres(nb_spheres, seed);
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

/// Generate random spheres within the unit cube centered around the origin
fn generate_uniform_spheres(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
) -> Vec<GeometricObject> {
    const EXPECTED_VOLUME: f64 = 0.025;
    let min_radius = (1. / 52. * FRAC_1_PI * EXPECTED_VOLUME / nb_spheres as f64).powf(1. / 3.);

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

#[derive(Serialize)]
struct ExperimentResults {
    nb_spheres: Vec<u32>,
    results: HashMap<String, Vec<Vec<usize>>>,
}
