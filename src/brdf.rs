use nalgebra::Vector3;
use std::f64;

use crate::film::RGB;
use crate::shade_rec::ShadeRec;

pub trait BRDF {
    fn f(&self, sr: &ShadeRec, wi: &Vector3<f64>, wo: &Vector3<f64>) -> RGB;
    fn sample_f(&self, _sr: &ShadeRec, _wo: &Vector3<f64>) -> (RGB, Vector3<f64>);
    fn rho(&self, sr: &ShadeRec, wo: &Vector3<f64>) -> RGB;
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
    fn f(&self, _sr: &ShadeRec, _wi: &Vector3<f64>, _wo: &Vector3<f64>) -> RGB {
        self.cd * (self.kd * f64::consts::FRAC_1_PI)
    }

    fn sample_f(&self, _sr: &ShadeRec, _wo: &Vector3<f64>) -> (RGB, Vector3<f64>) {
        unimplemented!()
    }

    fn rho(&self, _sr: &ShadeRec, _wo: &Vector3<f64>) -> RGB {
        self.cd * self.kd
    }
}
