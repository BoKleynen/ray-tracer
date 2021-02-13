use crate::film::FrameBuffer;
use crate::math::{OrthonormalBasis, Ray};
use crate::tracer::Tracer;
use crate::world::World;
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;
use std::f64;
use std::num::NonZeroUsize;
use std::ops::Neg;

#[derive(Debug, Copy, Clone)]
pub struct ViewPlane {
    pub horizontal_res: usize,
    pub vertical_res: usize,
    pub pixel_size: f64,
    pub gamma: f64,
    pub inv_gamma: f64,
}

pub trait Camera {
    fn render_scene<T: Tracer>(
        &self,
        world: &World,
        tracer: &T,
        view_plane: ViewPlane,
    ) -> FrameBuffer;
}

pub struct PerspectiveCamera {
    origin: Point3<f64>,
    basis: OrthonormalBasis,
    width: f64,
    height: f64,
    inv_x_res: f64,
    inv_y_res: f64,
}

impl PerspectiveCamera {
    fn generate_ray(&self, sample: (f64, f64)) -> Ray {
        let u = self.width * (sample.0 * self.inv_x_res - 0.5);
        let v = self.height * (sample.1 * self.inv_y_res - 0.5);

        let direction = &self.basis.u * u + &self.basis.v * v - &self.basis.w;

        Ray::new(self.origin.clone(), direction.into())
    }
}

impl Camera for PerspectiveCamera {
    fn render_scene<T: Tracer>(
        &self,
        _world: &World,
        tracer: &T,
        view_plane: ViewPlane,
    ) -> FrameBuffer {
        let mut buffer = FrameBuffer::new(view_plane.horizontal_res, view_plane.vertical_res);
        buffer
            .buffer()
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let x = (idx % view_plane.horizontal_res) as f64;
                let y = (idx / view_plane.vertical_res) as f64;
                let ray = self.generate_ray((x + 0.5, y + 0.5));

                pixel.add(tracer.trace_ray(&ray), 1.0);
            });

        buffer
    }
}

pub struct CameraBuilder {
    x_res: Option<NonZeroUsize>,
    y_res: Option<NonZeroUsize>,
    eye: Point3<f64>,
    look_at: Option<Vector3<f64>>,
    up: Option<Vector3<f64>>,
    fov: Option<f64>,
}

impl CameraBuilder {
    pub fn new(origin: Point3<f64>) -> Self {
        Self {
            x_res: None,
            y_res: None,
            eye: origin,
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
        self.look_at = Some(destination - self.eye);
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
            origin: self.eye,
            basis,
            inv_x_res,
            inv_y_res,
            width,
            height,
        };

        Some(camera)
    }
}
