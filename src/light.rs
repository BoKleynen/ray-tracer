use crate::film::RGB;
use crate::shade_rec::ShadeRec;
use nalgebra::{Point3, Vector3};

pub struct AmbientLight {
    ls: f64,
    color: RGB,
}

impl AmbientLight {
    pub fn new(ls: f64, color: RGB) -> Self {
        Self { ls, color }
    }

    pub fn radiance(&self) -> RGB {
        self.color * self.ls
    }
}

pub trait Light: Sync {
    fn direction(&self, sr: &ShadeRec) -> Vector3<f64>;
    fn radiance(&self, sr: &ShadeRec) -> RGB;
}

pub struct PointLight {
    ls: f64,
    color: RGB,
    location: Point3<f64>,
}

impl PointLight {
    pub fn new(ls: f64, color: RGB, location: Point3<f64>) -> Self {
        Self {
            ls,
            color,
            location,
        }
    }

    pub fn white(location: Point3<f64>) -> Self {
        Self {
            ls: 1.,
            color: RGB::white(),
            location,
        }
    }
}

impl Light for PointLight {
    fn direction(&self, sr: &ShadeRec) -> Vector3<f64> {
        (&self.location - &sr.hit_point).normalize()
    }

    fn radiance(&self, _sr: &ShadeRec) -> RGB {
        self.color * self.ls
    }
}
