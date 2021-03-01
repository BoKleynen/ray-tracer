use crate::film::RGB;
use crate::math::Ray;
use crate::shade_rec::ShadeRec;
use nalgebra::{Point3, Unit, Vector3};

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

pub trait Light: Sync {
    fn direction(&self, sr: &ShadeRec) -> Unit<Vector3<f64>>;
    fn radiance(&self, sr: &ShadeRec) -> RGB;
    fn visible(&self, ray: &Ray, sr: &ShadeRec) -> bool;
}

pub struct PointLight {
    // radiance scaling factor, in [0, +inf)
    ls: f64,
    color: RGB,
    location: Point3<f64>,
}

impl PointLight {
    pub fn new(ls: f64, color: RGB, location: Point3<f64>) -> Self {
        assert!(ls >= 0.);

        Self {
            ls,
            color,
            location,
        }
    }

    pub fn white(ls: f64, location: Point3<f64>) -> Self {
        Self {
            ls,
            color: RGB::white(),
            location,
        }
    }
}

impl Light for PointLight {
    fn direction(&self, sr: &ShadeRec) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.location - sr.hit_point)
    }

    fn radiance(&self, _sr: &ShadeRec) -> RGB {
        self.color * self.ls
    }

    fn visible(&self, ray: &Ray, sr: &ShadeRec) -> bool {
        !sr.world.shapes().iter().any(|shape| {
            shape
                .intersect(ray)
                .map_or(false, |hit| hit.t < (self.location - ray.origin()).norm())
        })
    }
}
