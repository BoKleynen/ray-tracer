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
    background_color: RGB,
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
                        material: shape.material(),
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

    pub fn ambient_light(&self) -> &AmbientLight {
        &self.ambient_light
    }

    pub fn background_color(&self) -> RGB {
        self.background_color
    }
}

pub struct WorldBuilder {
    shapes: Vec<Box<dyn Shape>>,
    lights: Vec<Box<dyn Light>>,
    ambient_light: Option<AmbientLight>,
    background_color: Option<RGB>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shape(mut self, shape: Box<dyn Shape>) -> Self {
        self.shapes.push(shape);
        self
    }

    pub fn light(mut self, light: Box<dyn Light>) -> Self {
        self.lights.push(light);
        self
    }

    pub fn background(mut self, color: RGB) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn build(self) -> Option<World> {
        let shapes = self.shapes;
        let lights = self.lights;
        let ambient_light = self
            .ambient_light
            .unwrap_or_else(|| AmbientLight::white(0.25));
        let background_color = self.background_color.unwrap_or_else(RGB::black);

        let world = World {
            shapes,
            lights,
            ambient_light,
            background_color,
        };

        Some(world)
    }
}

impl Default for WorldBuilder {
    fn default() -> Self {
        let shapes = Vec::new();
        let lights = Vec::new();
        let ambient_light = None;
        let background_color = None;

        Self {
            shapes,
            lights,
            ambient_light,
            background_color,
        }
    }
}
