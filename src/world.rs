use crate::film::RGB;
use crate::light::{AmbientLight, Light};
use crate::math::Ray;
use crate::shade_rec::ShadeRec;
use crate::shape::Shape;
use nalgebra::Vector3;

pub struct World {
    shapes: Vec<Box<dyn Shape>>,
    ambient_light: AmbientLight,
    lights: Vec<Box<dyn Light>>,
}

impl World {
    pub fn hit_objects(&self, ray: &Ray) -> Option<ShadeRec> {
        let mut sr: Option<ShadeRec> = None;
        let mut t_min = f64::INFINITY;

        self.shapes.iter().for_each(|shape| {
            if let Some(hit) = shape.intersect(&ray) {
                if hit.t < t_min {
                    t_min = hit.t;

                    sr = Some(ShadeRec {
                        hit_point: ray.origin() + hit.t * ray.direction(),
                        local_hit_point: hit.local_hit_point,
                        normal: hit.normal,
                        color: shape.color(),
                        ray: ray.clone(),
                        depth: 0,
                        direction: Vector3::default(),
                        world: self,
                    })
                }
            }
        });

        sr
    }

    pub fn shapes(&self) -> &[Box<dyn Shape>] {
        self.shapes.as_slice()
    }

    pub fn lights(&self) -> &[Box<dyn Light>] {
        self.lights.as_slice()
    }
}

pub struct WorldBuilder {
    shapes: Vec<Box<dyn Shape>>,
    lights: Vec<Box<dyn Light>>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        let shapes = Vec::new();
        let lights = Vec::new();

        Self { shapes, lights }
    }

    pub fn shape(mut self, shape: Box<dyn Shape>) -> Self {
        self.shapes.push(shape);
        self
    }

    pub fn light(mut self, light: Box<dyn Light>) -> Self {
        self.lights.push(light);
        self
    }

    pub fn build(self) -> Option<World> {
        let shapes = self.shapes;
        let lights = self.lights;
        let ambient_light = AmbientLight::new(1., RGB::new(1., 1., 1.));

        let world = World {
            shapes,
            lights,
            ambient_light,
        };
        Some(world)
    }
}
