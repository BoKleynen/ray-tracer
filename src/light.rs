use nalgebra::Unit;

use crate::film::Rgb;
use crate::material::{Emissive, Material};
use crate::math::Ray;
use crate::sampler::{Sampler, UniformSampler};
use crate::shade_rec::ShadeRec;
use crate::shape::{GeometricObject, Rectangle};
use crate::{Point, Vector, K_EPSILON};

pub struct AmbientLight {
    ls: f64,
    color: Rgb,
}

impl AmbientLight {
    pub fn new(ls: f64, color: Rgb) -> Self {
        Self { ls, color }
    }

    pub fn white(ls: f64) -> Self {
        let color = Rgb::white();
        Self { ls, color }
    }

    pub fn radiance(&self) -> Rgb {
        self.color * self.ls
    }
}

pub trait Light: Sync {
    fn average(&self, f: &dyn Fn(LightSample) -> Rgb) -> Rgb;
    fn radiance(&self, sr: &ShadeRec) -> Rgb;
    fn geometric_object(&self) -> Option<GeometricObject> {
        None
    }
}

impl<T: Light> Light for Box<T> {
    fn average(&self, f: &dyn Fn(LightSample) -> Rgb) -> Rgb {
        (**self).average(f)
    }

    fn radiance(&self, sr: &ShadeRec) -> Rgb {
        (**self).radiance(sr)
    }

    fn geometric_object(&self) -> Option<GeometricObject> {
        (**self).geometric_object()
    }
}

pub struct LightSample<'a> {
    light: &'a dyn Light,
    location: Point,
}

impl<'a> LightSample<'a> {
    pub fn direction(&self, sr: &ShadeRec) -> Unit<Vector> {
        Unit::new_normalize(self.location - sr.hit_point)
    }
}

impl<'a> LightSample<'a> {
    pub fn light(&self) -> &dyn Light {
        self.light
    }

    pub fn visible(&self, ray: &Ray, sr: &ShadeRec) -> bool {
        !sr.world.hit_any_object_where(ray, |hit| {
            hit.t < (self.location - ray.origin()).norm() - K_EPSILON
        })
    }
}

pub struct PointLight {
    location: Point,
    material: Emissive,
}

impl PointLight {
    pub fn new(ls: f64, color: Rgb, location: Point) -> Self {
        let material = Emissive::new(ls, color);

        Self { location, material }
    }

    pub fn white(ls: f64, location: Point) -> Self {
        Self::new(ls, Rgb::white(), location)
    }
}

impl Light for PointLight {
    fn average(&self, f: &dyn Fn(LightSample) -> Rgb) -> Rgb {
        let light_sample = LightSample {
            light: self,
            location: self.location,
        };

        f(light_sample)
    }

    fn radiance(&self, _sr: &ShadeRec) -> Rgb {
        self.material.ce * self.material.ls
    }
}

pub struct AreaLight {
    shape: Rectangle,
    material: Emissive,
    area: f64,
    sampler: UniformSampler,
}

impl AreaLight {
    pub fn new(shape: Rectangle, material: Emissive) -> Self {
        let area = shape.area();
        let sampler = UniformSampler::new(1);

        Self {
            shape,
            material,
            area,
            sampler,
        }
    }
}

impl Light for AreaLight {
    fn average(&self, f: &dyn Fn(LightSample) -> Rgb) -> Rgb {
        self.sampler.average(|sample| {
            let location = self.shape.sample(&sample);
            let light_sample = LightSample {
                light: self,
                location,
            };

            f(light_sample) / self.area
        })
    }

    fn radiance(&self, _sr: &ShadeRec) -> Rgb {
        self.material.ce * self.material.ls
    }

    fn geometric_object(&self) -> Option<GeometricObject> {
        Some(GeometricObject::new(
            Box::new(self.shape.clone()),
            Material::Emissive(self.material),
        ))
    }
}
