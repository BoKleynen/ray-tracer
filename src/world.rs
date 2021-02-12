use crate::camera::{Camera, PerspectiveCamera};
use crate::film::{FrameBuffer, RGB};
use crate::light::PointLight;
use crate::math::Ray;
use crate::shade_rec::ShadeRec;
use crate::shape::Shape;
use crate::tracer::Tracer;
use rayon::prelude::*;
use std::error::Error;
use std::num::NonZeroUsize;

pub struct World {
    shapes: Vec<Box<dyn Shape>>,
    camera: PerspectiveCamera,
    lights: Vec<PointLight>,
}

impl World {
    pub fn render_scene<T: Tracer>(
        &self,
        width: NonZeroUsize,
        height: NonZeroUsize,
        tracer: T,
    ) -> Result<FrameBuffer, Box<dyn Error>> {
        let mut buffer = FrameBuffer::new(width, height);
        buffer
            .buffer()
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let x = (idx % width.get()) as f64;
                let y = (idx / width.get()) as f64;
                let ray = self.camera.generate_ray((x + 0.5, y + 0.5));

                pixel.add(tracer.trace_ray(&ray), 1.0);
            });

        Ok(buffer)
    }

    // pub fn hit_objects(&self, ray: &Ray) -> ShadeRec {
    //     let mut sr = ShadeRec::new(&self);
    //     let mut t_min = f64::INFINITY;
    //
    //     // (t: f64, normal: Vector3, local_hit_point: Point3)
    //     self.shapes.iter().for_each(|shape| {
    //         if let Some(hit) = shape.intersect(ray) {
    //             if hit.t < t_min {
    //                 sr.hit_an_object = true;
    //                 t_min = hit.t;
    //                 sr.color = shape.color();
    //                 // TODO: calculate hit point
    //             }
    //         }
    //     });
    //
    //     sr
    // }

    pub fn shapes(&self) -> &[Box<dyn Shape>] {
        self.shapes.as_slice()
    }

    pub fn lights(&self) -> &[PointLight] {
        self.lights.as_slice()
    }
}

pub struct WorldBuilder {
    shapes: Vec<Box<dyn Shape>>,
    camera: Option<PerspectiveCamera>,
    lights: Vec<PointLight>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        let shapes = Vec::new();
        let camera = None;
        let lights = Vec::new();

        Self {
            shapes,
            camera,
            lights,
        }
    }

    pub fn shape(mut self, shape: Box<dyn Shape>) -> Self {
        self.shapes.push(shape);
        self
    }

    pub fn light(mut self, light: PointLight) -> Self {
        self.lights.push(light);
        self
    }

    pub fn camera(mut self, camera: PerspectiveCamera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn build(self) -> Option<World> {
        let shapes = self.shapes;
        let camera = self.camera?;
        let lights = self.lights;

        let world = World {
            shapes,
            camera,
            lights,
        };
        Some(world)
    }
}
