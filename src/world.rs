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
    lights: Vec<PointLight>,
}

impl World {
    pub fn hit_objects(&self, ray: &Ray) -> ShadeRec {
        let mut sr = ShadeRec::new(self);
        let mut t_min = f64::INFINITY;

        self.shapes.iter().for_each(|shape| {
            if let Some(hit) = shape.intersect(ray) {
                if hit.t < t_min {
                    sr.hit_an_object = true;
                    t_min = hit.t;
                    sr.color = shape.color()
                }
            }
        });

        sr
    }

    pub fn shapes(&self) -> &[Box<dyn Shape>] {
        self.shapes.as_slice()
    }

    pub fn lights(&self) -> &[PointLight] {
        self.lights.as_slice()
    }
}

pub struct WorldBuilder {
    shapes: Vec<Box<dyn Shape>>,
    lights: Vec<PointLight>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        let shapes = Vec::new();
        let lights = Vec::new();

        Self {
            shapes,
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

    pub fn build(self) -> Option<World> {
        let shapes = self.shapes;
        let lights = self.lights;

        let world = World {
            shapes,
            lights,
        };
        Some(world)
    }
}
