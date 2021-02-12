use crate::film::RGB;
use nalgebra::Point3;

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

    pub fn radiance(&self) -> RGB {
        self.color * self.ls
    }
}
