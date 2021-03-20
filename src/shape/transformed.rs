use crate::math::{Ray, Transformation};
use crate::shape::compound::Compound;
use crate::shape::obj::SmoothTriangle;
use crate::shape::{Cuboid, Hit, Obj, Plane, Shape, Sphere, AABB};
use itertools::Itertools;
use nalgebra::{min, Point3, Vector3};

pub struct Transformed<S> {
    shape: S,
    transformation: Transformation,
}

impl<S> Transformed<S> {
    pub fn new(shape: S, transformation: Transformation) -> Self {
        Self {
            shape,
            transformation,
        }
    }

    fn inverse_transform_normal(&self, normal: &Vector3<f64>) -> Vector3<f64> {
        self.transformation
            .inverse()
            .matrix()
            .transpose()
            .transform_vector(normal)
            .normalize()
    }

    fn transform_bounding_box(&self, aabb: AABB) -> AABB {
        let vertices = aabb
            .vertices()
            .iter()
            .map(|p| self.transformation.apply(p))
            .collect_vec();
        let min_x = vertices
            .iter()
            .map(|p| p.x)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_x = vertices
            .iter()
            .map(|p| p.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let min_y = vertices
            .iter()
            .map(|p| p.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_y = vertices
            .iter()
            .map(|p| p.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let min_z = vertices
            .iter()
            .map(|p| p.z)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_z = vertices
            .iter()
            .map(|p| p.z)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        AABB::new(
            Point3::new(min_x, min_y, min_z),
            Point3::new(max_x, max_y, max_z),
        )
    }
}

impl Transformed<Cuboid> {
    pub fn cuboid(corner: Point3<f64>, transformation: Transformation) -> Self {
        let shape = Cuboid::new(corner);
        Self {
            shape,
            transformation,
        }
    }
}

impl Transformed<Compound<SmoothTriangle>> {
    pub fn smooth_mesh(obj: Obj, transformation: Transformation) -> Self {
        let shape = obj.smooth();
        Self {
            shape,
            transformation,
        }
    }
}

impl Transformed<Plane> {
    pub fn plane(normal: Vector3<f64>, point: Point3<f64>, transformation: Transformation) -> Self {
        let shape = Plane::new(normal, point);
        Self {
            shape,
            transformation,
        }
    }
}

impl Transformed<Sphere> {
    pub fn sphere(transformation: Transformation) -> Self {
        let shape = Sphere::new();
        Self {
            shape,
            transformation,
        }
    }
}

impl<S: Shape> Shape for Transformed<S> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let inv_ray = self.transformation.apply_inverse(ray);
        self.shape.intersect(&inv_ray).map(|hit| {
            let normal = self.inverse_transform_normal(&hit.normal);

            Hit { normal, ..hit }
        })
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        let inv_ray = self.transformation.apply_inverse(ray);
        self.shape.count_intersection_tests(&inv_ray)
    }

    fn bbox(&self) -> AABB {
        self.transform_bounding_box(self.shape.bbox())
    }

    fn hit(&self, ray: &Ray) -> bool {
        let inv_ray = self.transformation.apply_inverse(ray);
        self.shape.hit(&inv_ray)
    }
}
