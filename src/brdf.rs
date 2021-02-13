use crate::shade_rec::ShadeRec;
use nalgebra::Vector3;
use crate::film::RGB;
use std::f64;

pub trait BRDF {
    fn f(&self, sr: &ShadeRec, wi: &Vector3<f64>, wo: &Vector3<f64>) -> RGB;
    fn sample_f(&self, sr: &ShadeRec, wi: &Vector3<f64>, wo: &Vector3<f64>) -> RGB;
    fn rho(&self, sr: &ShadeRec, wo: &Vector3<f64>) -> RGB;
}

pub struct Lambertian {
    kd: f64,
    cd: RGB
}

impl BRDF for Lambertian {
    fn f(&self, _sr: &ShadeRec, _wi: &Vector3<f64>, _wo: &Vector3<f64>) -> RGB {
        self.cd * (self.kd * f64::consts::FRAC_1_PI)
    }

    fn sample_f(&self, _sr: &ShadeRec, _wi: &Vector3<f64>, _wo: &Vector3<f64>) -> RGB {
        unimplemented!()
    }

    fn rho(&self, _sr: &ShadeRec, _wo: &Vector3<f64>) -> RGB {
        self.cd * self.kd
    }
}
