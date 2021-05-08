use std::f64;

use crate::film::RGB;
use crate::shade_rec::ShadeRec;
use crate::Vector;

pub trait BRDF {
    fn f(&self, sr: &ShadeRec, wi: &Vector, wo: &Vector) -> RGB;
    fn sample_f(&self, _sr: &ShadeRec, _wo: &Vector) -> (RGB, Vector);
    fn rho(&self, sr: &ShadeRec, wo: &Vector) -> RGB;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    // diffuse reflection coefficient, in [0, 1]
    kd: f64,
    // diffuse color
    cd: RGB,
}

impl Lambertian {
    pub fn new(kd: f64, cd: RGB) -> Self {
        assert!((0. ..=1.).contains(&kd));

        Lambertian { kd, cd }
    }
}

impl BRDF for Lambertian {
    fn f(&self, _sr: &ShadeRec, _wi: &Vector, _wo: &Vector) -> RGB {
        self.cd * (self.kd * f64::consts::FRAC_1_PI)
    }

    fn sample_f(&self, _sr: &ShadeRec, _wo: &Vector) -> (RGB, Vector) {
        unimplemented!()
    }

    fn rho(&self, _sr: &ShadeRec, _wo: &Vector) -> RGB {
        self.cd * self.kd
    }
}
