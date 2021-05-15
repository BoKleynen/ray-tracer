use cg_practicum::brdf::Lambertian;
use cg_practicum::film::Rgb;
use cg_practicum::material::Material;
use cg_practicum::math::Transformation;
use cg_practicum::shape::GeometricObject;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::Uniform;
use std::f64::consts::FRAC_1_PI;

pub fn generate_equal_spheres_uniform(
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

pub fn generate_uniform_spheres_uniform(
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

pub fn generate_spheres_skewed(
    _nb_spheres: u32,
    _seed: <ChaCha8Rng as SeedableRng>::Seed,
    _fill: f64,
) -> Vec<GeometricObject> {
    todo!()
}
