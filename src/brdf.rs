use std::f64::consts::FRAC_1_PI;

use crate::film::Rgb;
use crate::shade_rec::ShadeRec;
use crate::texture::Texture;
use crate::Vector;

pub trait Brdf {
    fn f(&self, sr: &ShadeRec, wi: &Vector, wo: &Vector) -> Rgb;
    fn sample_f(&self, _sr: &ShadeRec, _wo: &Vector) -> (Rgb, Vector);
    fn rho(&self, sr: &ShadeRec, wo: &Vector) -> Rgb;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    // diffuse reflection coefficient, in [0, 1]
    kd: f64,
    // diffuse color
    cd: Rgb,
}

impl Lambertian {
    pub fn new(kd: f64, cd: Rgb) -> Self {
        assert!((0. ..=1.).contains(&kd));

        Lambertian { kd, cd }
    }
}

impl Brdf for Lambertian {
    fn f(&self, _sr: &ShadeRec, _wi: &Vector, _wo: &Vector) -> Rgb {
        self.cd * (self.kd * FRAC_1_PI)
    }

    fn sample_f(&self, _sr: &ShadeRec, _wo: &Vector) -> (Rgb, Vector) {
        unimplemented!()
    }

    fn rho(&self, _sr: &ShadeRec, _wo: &Vector) -> Rgb {
        self.cd * self.kd
    }
}

pub struct SvLambertian {
    kd: f64,
    texture: Box<dyn Texture + Sync + Send>,
}

impl SvLambertian {
    pub fn new(kd: f64, texture: Box<dyn Texture + Sync + Send>) -> Self {
        assert!((0. ..=1.).contains(&kd));

        SvLambertian { kd, texture }
    }
}

impl Brdf for SvLambertian {
    fn f(&self, sr: &ShadeRec, _wi: &Vector, _wo: &Vector) -> Rgb {
        self.texture.get_color(sr) * self.kd * FRAC_1_PI
    }

    fn sample_f(&self, _sr: &ShadeRec, _wo: &Vector) -> (Rgb, Vector) {
        unimplemented!()
    }

    fn rho(&self, sr: &ShadeRec, _wo: &Vector) -> Rgb {
        self.texture.get_color(sr) * self.kd
    }
}
