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

    pub fn white(ls: f64) -> Self {
        let color = RGB::white();
        Self { ls, color }
    }

    pub fn radiance(&self) -> RGB {
        self.color * self.ls
    }
}

// TODO: use trait object or an enum
pub enum Light2 {
    Point {
        ls: f64,
        color: RGB,
        location: Point3<f64>,
    },
}

impl Light2 {
    pub fn direction(&self, sr: &ShadeRec) -> Vector3<f64> {
        match self {
            Light2::Point { location, .. } => (location - sr.hit_point).normalize(),
        }
    }

    pub fn radiance(&self, _sr: &ShadeRec) -> RGB {
        match self {
            Light2::Point { ls, color, .. } => *color * *ls,
        }
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
        (self.location - sr.hit_point).normalize()
    }

    fn radiance(&self, _sr: &ShadeRec) -> RGB {
        self.color * self.ls
    }
}
