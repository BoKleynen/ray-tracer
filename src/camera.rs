use crate::math::{OrthonormalBasis, Ray};
use nalgebra::{Point3, Vector3};
use std::f64;
use std::num::NonZeroUsize;
use std::ops::Neg;

pub trait Camera {
    fn generate_ray(&self, sample: (f64, f64)) -> Ray;
}

pub struct PerspectiveCamera {
    origin: Point3<f64>,
    basis: OrthonormalBasis,
    width: f64,
    height: f64,
    inv_x_res: f64,
    inv_y_res: f64,
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: (f64, f64)) -> Ray {
        let u = self.width * (sample.0 * self.inv_x_res - 0.5);
        let v = self.height * (sample.1 * self.inv_y_res - 0.5);

        let direction = &self.basis.u * u + &self.basis.v * v - &self.basis.w;

        Ray::new(self.origin.clone(), direction.into())
    }
}

pub struct CameraBuilder {
    x_res: Option<NonZeroUsize>,
    y_res: Option<NonZeroUsize>,
    origin: Point3<f64>,
    look_at: Option<Vector3<f64>>,
    up: Option<Vector3<f64>>,
    fov: Option<f64>,
}

impl CameraBuilder {
    pub fn new(origin: Point3<f64>) -> Self {
        Self {
            x_res: None,
            y_res: None,
            origin,
            look_at: None,
            up: None,
            fov: None,
        }
    }

    pub fn x_res(mut self, x_res: NonZeroUsize) -> Self {
        self.x_res = Some(x_res);
        self
    }

    pub fn y_res(mut self, y_res: NonZeroUsize) -> Self {
        self.y_res = Some(y_res);
        self
    }

    pub fn destination(mut self, destination: Point3<f64>) -> Self {
        self.look_at = Some(destination - self.origin);
        self
    }

    pub fn look_at(mut self, look_at: Vector3<f64>) -> Self {
        self.look_at = Some(look_at);
        self
    }

    pub fn fov(mut self, fov: f64) -> Self {
        if fov > 0.0 && fov < 180.0 {
            self.fov = Some(fov);
        }
        self
    }

    pub fn up(mut self, up: Vector3<f64>) -> Self {
        self.up = Some(up);
        self
    }

    pub fn build(self) -> Option<PerspectiveCamera> {
        let basis = OrthonormalBasis::from_vectors(&self.look_at?.neg(), &self.up?).unwrap();

        let inv_x_res = 1.0 / self.x_res?.get() as f64;
        let inv_y_res = 1.0 / self.y_res?.get() as f64;
        let width = 2.0 * (0.5 * self.fov?.to_radians()).tan();
        let height = (self.y_res?.get() as f64 * width) * inv_x_res;

        let camera = PerspectiveCamera {
            origin: self.origin,
            basis,
            inv_x_res,
            inv_y_res,
            width,
            height,
        };

        Some(camera)
    }
}
