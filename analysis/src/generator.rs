use cg_practicum::brdf::Lambertian;
use cg_practicum::film::Rgb;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::shape::{Bounded, GeometricObject, Obj, Transformed};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Beta, Normal, Uniform};
use std::f64::consts::FRAC_1_PI;
use std::sync::Arc;

const BUNNY_SCALE: f64 = 0.04;

pub fn equal_spheres_uniform(
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
        .collect()
}

pub fn uniform_spheres_uniform_position(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    let min_radius = (1. / 52. * FRAC_1_PI * fill / nb_spheres as f64).powf(1. / 3.);

    let mut rng = ChaCha8Rng::from_seed(seed);
    let radius_distribution = Uniform::new_inclusive(min_radius, 5. * min_radius);
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
        .collect()
}

pub fn equal_spheres_uniform_yz(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    let radius = (0.75 * FRAC_1_PI * fill / nb_spheres as f64).powf(1. / 3.);

    let mut rng = ChaCha8Rng::from_seed(seed);
    let x_distribution = Uniform::new_inclusive(-0.5, 0.5);
    let yz_distribution = Uniform::new_inclusive(-0.2, 0.2);
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_spheres)
        .map(|_| {
            let transformation =
                Transformation::scale(radius, radius, radius).then(&Transformation::translate(
                    rng.sample(x_distribution),
                    rng.sample(yz_distribution),
                    rng.sample(yz_distribution) - 1.,
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
        .collect()
}

pub fn equal_spheres_normal_yz(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    let radius = (0.75 * FRAC_1_PI * fill / nb_spheres as f64).powf(1. / 3.);

    let mut rng = ChaCha8Rng::from_seed(seed);
    let position_distribution = Uniform::new_inclusive(-0.5, 0.5);
    let normal = Normal::new(0., 0.25).unwrap();
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_spheres)
        .map(|_| {
            let transformation =
                Transformation::scale(radius, radius, radius).then(&Transformation::translate(
                    rng.sample(position_distribution),
                    // force values to lie within the unit cube.
                    // I am however unsure about the mathematical implications of this.
                    (&mut rng)
                        .sample_iter(&normal)
                        .find(|x| (-0.5..=0.5).contains(x))
                        .unwrap(),
                    (&mut rng)
                        .sample_iter(&normal)
                        .find(|x| (-0.5..=0.5).contains(x))
                        .unwrap()
                        - 1.,
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

pub fn equal_spheres_beta_corners(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    let radius = (0.75 * FRAC_1_PI * fill / nb_spheres as f64).powf(1. / 3.);

    let mut rng = ChaCha8Rng::from_seed(seed);
    let position_distribution = Beta::new(0.35, 0.35).unwrap();
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_spheres)
        .map(|_| {
            let transformation =
                Transformation::scale(radius, radius, radius).then(&Transformation::translate(
                    rng.sample(position_distribution) - 0.5,
                    rng.sample(position_distribution) - 0.5,
                    rng.sample(position_distribution) - 1.5,
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

pub fn equal_spheres_beta_x(
    nb_spheres: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
    fill: f64,
) -> Vec<GeometricObject> {
    let radius = (0.75 * FRAC_1_PI * fill / nb_spheres as f64).powf(1. / 3.);

    let mut rng = ChaCha8Rng::from_seed(seed);
    let beta = Beta::new(1.5, 2.5).unwrap();
    let uniform = Uniform::new_inclusive(-0.5, 0.5);
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_spheres)
        .map(|_| {
            let transformation =
                Transformation::scale(radius, radius, radius).then(&Transformation::translate(
                    rng.sample(beta) - 0.5,
                    rng.sample(uniform),
                    rng.sample(uniform) - 1.,
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

pub fn instanced_bunnies_uniform(
    nb_bunnies: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
) -> Vec<GeometricObject> {
    let mesh = Arc::new(Obj::load("../models/bunny.obj").unwrap().smooth());
    let mut rng = ChaCha8Rng::from_seed(seed);
    let position_distribution = Uniform::new_inclusive(-0.5, 0.5);
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_bunnies)
        .map(|_| {
            let transformation = Transformation::scale(BUNNY_SCALE, BUNNY_SCALE, BUNNY_SCALE).then(
                &Transformation::translate(
                    rng.sample(position_distribution),
                    rng.sample(position_distribution),
                    rng.sample(position_distribution) - 1.,
                ),
            );
            let color = Rgb::new(
                rng.sample(color_distribution),
                rng.sample(color_distribution),
                rng.sample(color_distribution),
            );
            let material = Material::Matte {
                ambient_brdf: Lambertian::new(0.15, color),
                diffuse_brdf: Lambertian::new(0.75, color),
            };

            let transformed = Transformed::new(mesh.clone(), transformation);

            GeometricObject::new(Box::new(transformed), material)
        })
        .collect_vec()
}

pub fn instanced_bunnies_non_overlapping(
    nb_bunnies: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
) -> Vec<GeometricObject> {
    let mesh = Arc::new(Obj::load("../models/bunny.obj").unwrap().smooth());
    // make sure the bounding boxes don't touch each other.
    let bunny_width = (mesh.bbox().p1 - mesh.bbox().p0).x * 1.01;
    let scale = 1. / (bunny_width * nb_bunnies as f64);
    let bunny_width = bunny_width * scale;
    let mut rng = ChaCha8Rng::from_seed(seed);
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_bunnies)
        .map(|i| {
            let transformation =
                Transformation::scale(scale, scale, scale).then(&Transformation::translate(
                    (-0.5 + bunny_width / 2.) + i as f64 * bunny_width,
                    0.,
                    -1.,
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

            let transformed = Transformed::new(mesh.clone(), transformation);

            GeometricObject::new(Box::new(transformed), material)
        })
        .collect_vec()
}

pub fn flattened_bunnies_uniform(
    nb_bunnies: u32,
    seed: <ChaCha8Rng as SeedableRng>::Seed,
) -> Vec<GeometricObject> {
    let obj = Obj::load("../models/bunny.obj").unwrap();
    let mut rng = ChaCha8Rng::from_seed(seed);
    let position_distribution = Uniform::new_inclusive(-0.5, 0.5);
    let color_distribution = Uniform::new_inclusive(0., 1.);

    (0..nb_bunnies)
        .flat_map(|_| {
            let transformation = Transformation::scale(BUNNY_SCALE, BUNNY_SCALE, BUNNY_SCALE).then(
                &Transformation::translate(
                    rng.sample(position_distribution),
                    rng.sample(position_distribution),
                    rng.sample(position_distribution) - 1.,
                ),
            );
            let color = Rgb::new(
                rng.sample(color_distribution),
                rng.sample(color_distribution),
                rng.sample(color_distribution),
            );

            obj.transform(&transformation)
                .smooth_triangles()
                .into_iter()
                .map(|triangle| {
                    let material = Material::Matte {
                        ambient_brdf: Lambertian::new(0.15, color),
                        diffuse_brdf: Lambertian::new(0.75, color),
                    };

                    GeometricObject::new(Box::new(triangle), material)
                })
                .collect_vec()
        })
        .collect_vec()
}
