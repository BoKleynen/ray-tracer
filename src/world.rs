use crate::bvh::SplittingHeuristic;
use crate::film::Rgb;
use crate::light::{AmbientLight, Light};
use crate::math::Ray;
use crate::shade_rec::ShadeRec;
use crate::shape::{Compound, GeometricObject, Hit, Intersect};
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
            // safety: since shape is in the world, this reference will at least be valid within
            // this function.

            ShadeRec {
                hit_point: ray.origin() + hit.t * ray.direction(),
                local_hit_point: hit.local_hit_point,
                uv: hit.uv,
                normal: hit.normal,
                shape: hit.shape,
                direction: Vector::default(),
                world: self,
            }
        })
    }

    pub fn hit_any_object_where<P>(&self, ray: &Ray, p: P) -> bool
    where
        P: Fn(Hit<&GeometricObject>) -> bool,
    {
        self.geometric_objects.intersect_any_where(ray, |hit| {
            let hit = Hit {
                t: hit.t,
                normal: hit.normal,
                local_hit_point: hit.local_hit_point,
                // safety: since shape is in the world, this reference will at least be valid within
                // this function.
                shape: unsafe { hit.shape.as_ref() },
                uv: hit.uv,
            };

            p(hit)
        })
    }

    pub fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.geometric_objects.count_intersection_tests(ray)
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
    splitting_heuristic: Option<SplittingHeuristic>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn geometric_object(mut self, geometric_object: GeometricObject) -> Self {
        self.geometric_objects.push(geometric_object);
        self
    }

    pub fn geometric_objects(mut self, mut geometric_objects: Vec<GeometricObject>) -> Self {
        self.geometric_objects.append(&mut geometric_objects);
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

    pub fn splitting_heuristic(mut self, splitting_heuristic: SplittingHeuristic) -> Self {
        self.splitting_heuristic = Some(splitting_heuristic);
        self
    }

    pub fn build(self) -> Option<World> {
        let mut geometric_objects = self.geometric_objects;
        geometric_objects.extend(
            self.lights
                .iter()
                .filter_map(|light| light.geometric_object()),
        );
        let geometric_objects = Compound::new_with_splitting_heuristic(
            geometric_objects,
            self.splitting_heuristic.unwrap_or_default(),
        );
        let lights = self.lights;
        let ambient_light = self
            .ambient_light
            .unwrap_or_else(|| AmbientLight::white(0.25));
        let background_color = self.background_color.unwrap_or_else(Rgb::black);

        let world = World {
            geometric_objects,
            ambient_light,
            lights,
            background_color,
        };

        Some(world)
    }
}

impl Default for WorldBuilder {
    fn default() -> Self {
        let geometric_objects = Vec::new();
        let lights = Vec::new();
        let ambient_light = None;
        let background_color = None;
        let splitting_heuristic = None;

        Self {
            geometric_objects,
            lights,
            ambient_light,
            background_color,
            splitting_heuristic,
        }
    }
}
