use crate::film::RGB;
use crate::material::{Emissive, Material};
use crate::math::Ray;
use crate::sampler::{Sampler, UniformSampler};
use crate::shade_rec::ShadeRec;
use crate::shape::{GeometricObject, Rectangle, Shape};
use crate::K_EPSILON;
use nalgebra::{Point3, Unit, Vector3};

pub struct AmbientLight {
    ls: f64,
    color: RGB,
}

impl AmbientLight {
    pub fn new(ls: f64, color: RGB) -> Self {
        Self { ls, color }
    }

    pub fn white(ls: f64) -> Self {
        let color = RGB::white();
        Self { ls, color }
    }

    pub fn radiance(&self) -> RGB {
        self.color * self.ls
    }
}

pub trait Light: Sync {
    fn average(&self, f: &dyn Fn(LightSample) -> RGB) -> RGB;
    fn radiance(&self, sr: &ShadeRec) -> RGB;
    fn geometric_object(&self) -> Option<GeometricObject> {
        None
    }
}

impl<T: Light> Light for Box<T> {
    fn average(&self, f: &dyn Fn(LightSample) -> RGB) -> RGB {
        (**self).average(f)
    }

    fn radiance(&self, sr: &ShadeRec) -> RGB {
        (**self).radiance(sr)
    }

    fn geometric_object(&self) -> Option<GeometricObject> {
        (**self).geometric_object()
    }
}

pub struct LightSample<'a> {
    light: &'a dyn Light,
    location: Point3<f64>,
    normal: Unit<Vector3<f64>>,
}

impl<'a> LightSample<'a> {
    pub fn direction(&self, sr: &ShadeRec) -> Unit<Vector3<f64>> {
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
    location: Point3<f64>,
    material: Emissive,
}

impl PointLight {
    pub fn new(ls: f64, color: RGB, location: Point3<f64>) -> Self {
        let material = Emissive::new(ls, color);

        Self { material, location }
    }

    pub fn white(ls: f64, location: Point3<f64>) -> Self {
        Self::new(ls, RGB::white(), location)
    }
}

impl Light for PointLight {
    fn average(&self, f: &dyn Fn(LightSample) -> RGB) -> RGB {
        unimplemented!()
    }

    fn radiance(&self, _sr: &ShadeRec) -> RGB {
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
        let sampler = UniformSampler::new(2);

        Self {
            shape,
            material,
            area,
            sampler,
        }
    }
}

impl Light for AreaLight {
    fn average(&self, f: &dyn Fn(LightSample) -> RGB) -> RGB {
        self.sampler.average(|sample| {
            let location = self.shape.sample(&sample);
            let normal = self.shape.normal_at(&location);
            let light_sample = LightSample {
                light: self,
                location,
                normal,
            };

            f(light_sample) / self.area
        })
    }

    fn radiance(&self, sr: &ShadeRec) -> RGB {
        self.material.ce * self.material.ls
    }

    fn geometric_object(&self) -> Option<GeometricObject> {
        Some(GeometricObject::new(
            Box::new(self.shape.clone()),
            Material::Emissive(self.material.clone()),
        ))
    }
}
