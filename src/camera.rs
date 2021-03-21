use crate::math::{OrthonormalBasis, Ray};
use crate::sampler::Sample;
use nalgebra::{Point3, Vector3};
use std::f64;
use std::ops::Neg;

pub trait Camera {
    fn generate_ray(&self, column: usize, row: usize, sample: Sample) -> Ray;

    fn resolution(&self) -> (usize, usize);
}

#[derive(Debug)]
pub struct PerspectiveCamera {
    x_res: usize,
    y_res: usize,
    origin: Point3<f64>,
    basis: OrthonormalBasis,
    width: f64,
    height: f64,
    inv_x_res: f64,
    inv_y_res: f64,
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, column: usize, row: usize, sample: (f64, f64)) -> Ray {
        let (sample_x, sample_y) = sample;

        let xv = self.width * ((column as f64 + sample_x) * self.inv_x_res - 0.5);
        let yv = self.height * ((row as f64 + sample_y) * self.inv_y_res - 0.5);

        let direction = *self.basis.u * xv + *self.basis.v * yv - *self.basis.w;

        Ray::new(self.origin, direction)
    }

    fn resolution(&self) -> (usize, usize) {
        (self.x_res, self.y_res)
    }
}

pub struct CameraBuilder {
    x_res: Option<usize>,
    y_res: Option<usize>,
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

    pub fn x_res(mut self, x_res: usize) -> Self {
        self.x_res = Some(x_res);
        self
    }

    pub fn y_res(mut self, y_res: usize) -> Self {
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
        if fov > 0. && fov < 180. {
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

        let x_res = self.x_res?;
        let y_res = self.y_res?;
        let inv_x_res = 1. / x_res as f64;
        let inv_y_res = 1. / y_res as f64;
        let width = 2. * (0.5 * self.fov?.to_radians()).tan();
        let height = (y_res as f64 * width) * inv_x_res;

        let camera = PerspectiveCamera {
            x_res,
            y_res,
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
