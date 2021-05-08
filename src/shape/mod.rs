use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::sampler::{Sample, Sampler};
use crate::shape::transformed::Transformed;

use crate::{Point, Vector};
pub use aabb::AABB;
pub use cuboid::Cuboid;
pub use obj::{FlatTriangle, Obj, SmoothTriangle};
pub use plane::Plane;
pub use rectangle::Rectangle;
pub use sphere::Sphere;

mod aabb;
mod compound;
mod cuboid;
mod obj;
mod plane;
mod rectangle;
mod sphere;
mod transformed;

pub struct GeometricObject {
    shape: Box<dyn Shape>,
    material: Material,
}

impl GeometricObject {
    pub fn new(shape: Box<dyn Shape>, material: Material) -> Self {
        Self { shape, material }
    }

    pub fn shape(&self) -> &dyn Shape {
        self.shape.as_ref()
    }

    pub fn material(&self) -> Material {
        self.material.clone()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape.intersect(ray)
    }

    pub fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.shape.count_intersection_tests(ray)
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        self.shape.hit(ray)
    }

    pub fn sphere(transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(Transformed::sphere(transformation));
        Self::new(shape, material)
    }

    pub fn cuboid(corner: Point, transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(Transformed::cuboid(corner, transformation));
        Self::new(shape, material)
    }

    pub fn plane(
        normal: Vector,
        point: Point,
        transformation: Transformation,
        material: Material,
    ) -> Self {
        let shape = Box::new(Transformed::plane(normal, point, transformation));
        Self::new(shape, material)
    }

    pub fn triangle_mesh(obj: Obj, transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(Transformed::smooth_mesh(obj, transformation));
        Self::new(shape, material)
    }
}

pub trait Bounded {
    fn bbox(&self) -> AABB;
}

impl<T: Bounded + ?Sized> Bounded for Box<T> {
    #[inline]
    fn bbox(&self) -> AABB {
        (**self).bbox()
    }
}

pub trait Shape: Bounded + Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn count_intersection_tests(&self, ray: &Ray) -> usize;

    fn hit(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

impl<T: Shape + ?Sized> Shape for Box<T> {
    #[inline]
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        (**self).intersect(ray)
    }

    #[inline]
    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        (**self).count_intersection_tests(ray)
    }

    #[inline]
    fn hit(&self, ray: &Ray) -> bool {
        (**self).hit(ray)
    }
}

pub trait SampleShape: Shape {
    fn average<B, S: Sampler, F: Fn(Point) -> B>(&self) -> B;
}

pub struct Hit {
    pub t: f64,
    pub normal: Vector,
    pub local_hit_point: Point,
}
