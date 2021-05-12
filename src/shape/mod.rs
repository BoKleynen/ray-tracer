use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::{Point3, Vector, Point2};
use std::ptr::NonNull;

pub use aabb::{Aabb, Union};
pub use compound::Compound;
pub use cuboid::Cuboid;
pub use obj::{FlatTriangle, Obj, SmoothTriangle};
pub use plane::Plane;
pub use rectangle::Rectangle;
pub use sphere::Sphere;
use std::ops::Deref;
pub use transformed::Transformed;

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

impl<T, S> Bounded for T
where
    S: Bounded + ?Sized,
    T: Deref<Target = S>,
{
    fn bbox(&self) -> Aabb {
        (**self).bbox()
    }
}

pub trait Intersect: Bounded {
    type Intersection;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>>;

    fn count_intersection_tests(&self, ray: &Ray) -> usize;

    fn hit(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

impl<T, S> Intersect for T
where
    S: Intersect + ?Sized,
    T: Deref<Target = S>,
{
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

impl Intersect for GeometricObject {
    // We can't use `&'a Self`, because GAT aren't implemented yet.
    type Intersection = NonNull<Self>;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>> {
        self.shape.intersect(ray).map(|hit| Hit {
            t: hit.t,
            normal: hit.normal,
            local_hit_point: hit.local_hit_point,
            shape: self.into(),
            uv: hit.uv,
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

    pub fn cuboid(corner: Point3, transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(Transformed::cuboid(corner, transformation));
        Self::new(shape, material)
    }

    pub fn plane(
        normal: Vector,
        point: Point3,
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

pub struct Hit<S> {
    pub t: f64,
    pub normal: Vector,
    pub local_hit_point: Point3,
    pub shape: S,
    pub uv: Point2,
}
