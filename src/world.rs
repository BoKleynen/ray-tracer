use crate::film::Rgb;
use crate::light::{AmbientLight, Light};
use crate::math::Ray;
use crate::shade_rec::ShadeRec;
use crate::shape::{Compound, GeometricObject, Hit, Shape, Intersect};
use crate::Vector;

pub struct World {
    geometric_objects: Compound<GeometricObject>,
    ambient_light: AmbientLight,
    lights: Vec<Box<dyn Light>>,
    background_color: Rgb,
}

impl World {
    pub fn hit_objects(&self, ray: &Ray) -> Option<ShadeRec> {
        self.geometric_objects.intersect(&ray).map(|hit| {
            let shape = unsafe { hit.shape.as_ref() };

            ShadeRec {
                hit_point: ray.origin() + hit.t * ray.direction(),
                local_hit_point: hit.local_hit_point,
                normal: hit.normal,
                material: shape.material(),
                depth: 0,
                direction: Vector::default(),
                world: self,
            }
        })
    }

    pub fn hit_any_object_where<F>(&self, ray: &Ray, f: F) -> bool
    where
        F: Fn(Hit<&GeometricObject>) -> bool,
    {
        self.geometric_objects.intersect(ray).map_or(false, |hit| {
            let hit = Hit {
                t: hit.t,
                normal: hit.normal,
                local_hit_point: hit.local_hit_point,
                shape: unsafe { hit.shape.as_ref() }
            };

            f(hit)
        })
    }

    pub fn geometric_objects(&self) -> &Compound<GeometricObject> {
        &self.geometric_objects
    }

    pub fn lights(&self) -> &[Box<dyn Light>] {
        self.lights.as_slice()
    }

    pub fn ambient_light(&self) -> &AmbientLight {
        &self.ambient_light
    }

    pub fn background_color(&self) -> Rgb {
        self.background_color
    }
}

pub struct WorldBuilder {
    geometric_objects: Vec<GeometricObject>,
    lights: Vec<Box<dyn Light>>,
    ambient_light: Option<AmbientLight>,
    background_color: Option<Rgb>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn geometric_object(mut self, geometric_object: GeometricObject) -> Self {
        self.geometric_objects.push(geometric_object);
        self
    }

    pub fn light(mut self, light: Box<dyn Light>) -> Self {
        self.lights.push(light);
        self
    }

    pub fn background(mut self, color: Rgb) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn build(self) -> Option<World> {
        let mut geometric_objects = self.geometric_objects;
        geometric_objects.extend(
            self.lights
                .iter()
                .filter_map(|light| light.geometric_object()),
        );
        let lights = self.lights;
        let ambient_light = self
            .ambient_light
            .unwrap_or_else(|| AmbientLight::white(0.25));
        let background_color = self.background_color.unwrap_or_else(Rgb::black);

        let world = World {
            geometric_objects: Compound::new(geometric_objects),
            ambient_light,
            lights,
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
            geometric_objects: shapes,
            lights,
            ambient_light,
            background_color,
        }
    }
}
