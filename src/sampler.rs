use crate::film::RGB;
use itertools::Itertools;
use std::iter::{once, Once};
use std::marker::PhantomData;

pub type Sample = (f64, f64);

pub trait Sampler {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB;
}

#[derive(Debug, Copy, Clone)]
pub struct Unsampled {}

impl Unsampled {
    pub fn new() -> Self {
        Self {}
    }
}

impl Sampler for Unsampled {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB {
        f((0.5, 0.5))
    }
}

pub struct RegularGrid {
    samples: Vec<Sample>,
}

impl RegularGrid {
    pub fn new(nb_samples: usize) -> Self {
        let n = (nb_samples as f64).sqrt();
        let inv_n = 1. / n;
        let n = n as usize;

        let samples = (0..n)
            .cartesian_product((0..n))
            .map(|(p, q)| {
                let p = p as f64;
                let q = q as f64;

                ((p + 0.5) * inv_n, (q + 0.5) * inv_n)
            })
            .collect();

        Self { samples }
    }
}

impl Sampler for RegularGrid {
    fn average<F: Fn(Sample) -> RGB>(&self, f: F) -> RGB {
        self.samples.iter().map(|&sample| f(sample)).sum::<RGB>() / self.samples.len() as f64
    }
}
