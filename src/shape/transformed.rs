use itertools::Itertools;

use crate::math::{Ray, Transformation};
use crate::shape::compound::Compound;
use crate::shape::obj::SmoothTriangle;
use crate::shape::{Aabb, Bounded, Cuboid, Hit, Intersect, Obj, Plane, Sphere};
use crate::{Point3, Vector};

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

    fn inverse_transform_normal(&self, normal: &Vector) -> Vector {
        self.transformation
            .inverse()
            .matrix()
            .transpose()
            .transform_vector(normal)
            .normalize()
    }

    fn transform_bounding_box(&self, aabb: Aabb) -> Aabb {
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

        Aabb::new(
            Point3::new(min_x, min_y, min_z),
            Point3::new(max_x, max_y, max_z),
        )
    }
}

impl Transformed<Cuboid> {
    pub fn cuboid(corner: Point3, transformation: Transformation) -> Self {
        let shape = Cuboid::new(corner);
        Self::new(shape, transformation)
    }
}

impl Transformed<Compound<SmoothTriangle>> {
    pub fn smooth_mesh(obj: Obj, transformation: Transformation) -> Self {
        let shape = obj.smooth();
        Self::new(shape, transformation)
    }
}

impl Transformed<Plane> {
    pub fn plane(normal: Vector, point: Point3, transformation: Transformation) -> Self {
        let shape = Plane::new(normal, point);
        Self::new(shape, transformation)
    }
}

impl Transformed<Sphere> {
    pub fn sphere(transformation: Transformation) -> Self {
        let shape = Sphere::new();
        Self::new(shape, transformation)
    }
}

impl<S: Bounded> Bounded for Transformed<S> {
    fn bbox(&self) -> Aabb {
        self.transform_bounding_box(self.shape.bbox())
    }
}

impl<S: Intersect> Intersect for Transformed<S> {
    type Intersection = S::Intersection;

    fn intersect(&self, ray: &Ray) -> Option<Hit<Self::Intersection>> {
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

    fn hit(&self, ray: &Ray) -> bool {
        let inv_ray = self.transformation.apply_inverse(ray);
        self.shape.hit(&inv_ray)
    }
}
