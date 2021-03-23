use crate::film::RGB;
use itertools::Itertools;
use rand::prelude::*;

pub type Sample = (f64, f64);

pub trait Sampler {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB;
}

#[derive(Debug, Copy, Clone)]
pub struct Unsampled {}

impl Default for Unsampled {
    fn default() -> Self {
        Self {}
    }
}

impl Sampler for Unsampled {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB {
        f((0.5, 0.5))
    }
}

pub struct RegularSampler {
    samples: Vec<Sample>,
}

impl RegularSampler {
    pub fn new(nb_samples: usize) -> Self {
        let n = (nb_samples as f64).sqrt();
        let inv_n = 1. / n;
        let n = n as usize;

        let samples = (0..n)
            .cartesian_product(0..n)
            .map(|(p, q)| {
                let p = p as f64;
                let q = q as f64;

                ((p + 0.5) * inv_n, (q + 0.5) * inv_n)
            })
            .collect();

        Self { samples }
    }
}

impl Sampler for RegularSampler {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB {
        self.samples.iter().map(|&sample| f(sample)).sum::<RGB>() / self.samples.len() as f64
    }
}

pub struct JitteredSampler {
    nb_samples: usize,
}

impl JitteredSampler {
    pub fn new(nb_samples: usize) -> Self {
        Self { nb_samples }
    }
}

impl Sampler for JitteredSampler {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB {
        let n = (self.nb_samples as f64).sqrt();
        let inv_n = 1. / n;
        let n = n as usize;

        (0..n)
            .cartesian_product(0..n)
            .map(|(p, q)| {
                let x = (p as f64 + thread_rng().gen::<f64>()) * inv_n;
                let y = (q as f64 + thread_rng().gen::<f64>()) * inv_n;

                f((x, y))
            })
            .sum::<RGB>()
            / self.nb_samples as f64
    }
}

pub struct UniformSampler {
    nb_samples: usize,
}

impl UniformSampler {
    pub fn new(nb_samples: usize) -> Self {
        Self { nb_samples }
    }
}

impl Sampler for UniformSampler {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB {
        std::iter::repeat_with(|| thread_rng().gen::<(f64, f64)>())
            .take(self.nb_samples)
            .map(f)
            .sum::<RGB>()
            / self.nb_samples as f64
    }
}
