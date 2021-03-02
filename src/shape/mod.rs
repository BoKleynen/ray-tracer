mod aabb;
mod cuboid;
mod obj;
mod plane;
mod sphere;

pub use cuboid::Cuboid;
pub use obj::{Obj, TriangleMesh};
pub use plane::Plane;
pub use sphere::Sphere;

use crate::material::Material;
use crate::math::{Ray, Transformation};
use image::imageops::FilterType::Triangle;
use nalgebra::{Point3, Vector3};

pub struct GeometricObject {
    shape: Box<dyn Shape>,
    material: Material,
}

impl GeometricObject {
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
        let shape = Box::new(Sphere::new(transformation));

        Self { shape, material }
    }

    pub fn cuboid(corner: Point3<f64>, transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(Cuboid::new(corner, transformation));

        Self { shape, material }
    }

    pub fn plane(
        normal: Vector3<f64>,
        point: Point3<f64>,
        transformation: Transformation,
        material: Material,
    ) -> Self {
        let shape = Box::new(Plane::new(normal, point, transformation));

        Self { shape, material }
    }

    pub fn triangle_mesh(obj: Obj, transformation: Transformation, material: Material) -> Self {
        let shape = Box::new(TriangleMesh::new(obj, transformation));

        Self { shape, material }
    }
}

// impl GeometricObject<Sphere> {
//     pub fn sphere(transformation: Transformation, material: Material) -> Self {
//         let shape = Box::new(Sphere::new(transformation));
//
//         Self { shape, material }
//     }
// }
//
// impl GeometricObject<Cuboid> {
//     pub fn cuboid(corner: Point3<f64>, transformation: Transformation, material: Material) -> Self {
//         let shape = Box::new(Cuboid::new(corner, transformation));
//
//         Self { shape, material }
//     }
// }
//
// impl GeometricObject<Plane> {
//     pub fn plane(normal: Vector3<f64>,
//                  point: Point3<f64>,
//                  transformation: Transformation, material: Material) -> Self {
//         let shape = Box::new(Plane::new(normal, point, transformation));
//
//         Self { shape, material }
//     }
// }
//
// impl GeometricObject<TriangleMesh> {
//     pub fn triangle_mesh(obj: Obj, transformation: Transformation, material: Material) -> Self {
//         let shape = Box::new(TriangleMesh::new(obj, transformation));
//
//         Self { shape, material }
//     }
// }

pub trait Shape: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn count_intersection_tests(&self, ray: &Ray) -> usize;

    fn hit(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

pub struct Hit {
    pub t: f64,
    pub normal: Vector3<f64>,
    pub local_hit_point: Point3<f64>,
}
