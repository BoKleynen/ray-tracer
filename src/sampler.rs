use std::iter::{Once, once};

pub type Sample = (f64, f64);

pub trait Sampler<I: IntoIterator<Item = Sample>> {
    fn samples(&self) -> I;
    fn nb_samples(&self) -> usize;
}

#[derive(Debug, Copy, Clone)]
pub struct Unsampled {}

impl Unsampled {
    pub fn new() -> Self {
        Self {}
    }
}

impl Sampler<std::iter::Once<Sample>> for Unsampled {
    fn samples(&self) -> Once<(f64, f64)> {
        once((0.5, 0.5))
    }

    fn nb_samples(&self) -> usize {
        1
    }
}
