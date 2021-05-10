use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::sampler::Sampler;
use crate::{Point, Vector};

pub use aabb::Aabb;
pub use compound::Compound;
pub use cuboid::Cuboid;
pub use obj::{FlatTriangle, Obj, SmoothTriangle};
pub use plane::Plane;
pub use rectangle::Rectangle;
pub use sphere::Sphere;
pub use transformed::Transformed;
use std::ops::Deref;
use std::ptr::NonNull;

mod aabb;
mod compound;
mod cuboid;
mod obj;
mod plane;
mod rectangle;
mod sphere;
mod transformed;

pub trait Bounded {
    fn bbox(&self) -> Aabb;
}

pub trait Intersect: Bounded {
    type Intersection;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>>;

    fn count_intersection_tests(&self, ray: &Ray) -> usize;

    fn hit(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

// marker trait
pub trait Shape: Intersect<Intersection = ()> + Sync {}

impl<S: Intersect<Intersection = ()> + Sync> Shape for S {}

pub struct GeometricObject {
    shape: Box<dyn Shape>,
    material: Material,
}

impl Bounded for GeometricObject {
    fn bbox(&self) -> Aabb {
        self.shape.bbox()
    }
}

impl<'a> Intersect for GeometricObject {
    type Intersection = NonNull<Self>;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>> {
        self.shape.intersect(ray).map(|hit| Hit {
            t: hit.t,
            normal: Default::default(),
            local_hit_point: hit.local_hit_point,
            shape: self.into(),
        })
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        self.shape.count_intersection_tests(ray)
    }

    fn hit(&self, ray: &Ray) -> bool {
        self.shape.hit(ray)
    }
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


impl<T: Bounded + ?Sized> Bounded for Box<T> {
    #[inline]
    fn bbox(&self) -> Aabb {
        (**self).bbox()
    }
}

impl<S: Intersect + ?Sized> Intersect for Box<S> {
    type Intersection = S::Intersection;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>> {
        (**self).intersect(ray)
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        (**self).count_intersection_tests(ray)
    }

    fn hit(&self, ray: &Ray) -> bool {
        (**self).hit(ray)
    }
}

pub struct Hit<S> {
    pub t: f64,
    pub normal: Vector,
    pub local_hit_point: Point,
    pub shape: S,
}
