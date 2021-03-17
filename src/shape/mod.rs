mod aabb;
mod compound;
mod cuboid;
mod obj;
mod plane;
mod sphere;
mod transformed;

pub use aabb::AABB;
pub use cuboid::Cuboid;
pub use obj::{FlatTriangle, Obj, SmoothTriangle};
pub use plane::Plane;
pub use sphere::Sphere;

use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::transformed::Transformed;
use nalgebra::{Point3, Vector3};

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

        Self { shape, material }
    }

    pub fn cuboid(corner: Point3<f64>, transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(Transformed::cuboid(corner, transformation));

        Self { shape, material }
    }

    pub fn plane(
        normal: Vector3<f64>,
        point: Point3<f64>,
        transformation: Transformation,
        material: Material,
    ) -> Self {
        let shape = Box::new(Transformed::plane(normal, point, transformation));

        Self { shape, material }
    }

    pub fn triangle_mesh(obj: Obj, transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(Transformed::smooth_mesh(obj, transformation));

        Self { shape, material }
    }
}

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn count_intersection_tests(&self, ray: &Ray) -> usize;

    fn bounding_box(&self) -> AABB;

    fn hit(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

pub struct Hit {
    pub t: f64,
    pub normal: Vector3<f64>,
    pub local_hit_point: Point3<f64>,
}
