use crate::math::{Ray, Transformation};
use crate::shape::compound::Compound;
use crate::shape::obj::SmoothTriangle;
use crate::shape::{Cuboid, Hit, Obj, Plane, Shape, Sphere, AABB};
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
        let AABB { p0: p1, p1: p2 } = aabb;
        let p1 = self.transformation.apply(&p1);
        let p2 = self.transformation.apply(&p2);

        let (x_min, x_max) = if p1.x < p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };
        let (y_min, y_max) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };
        let (z_min, z_max) = if p1.z < p2.z {
            (p1.z, p2.z)
        } else {
            (p2.z, p1.z)
        };

        AABB::new(
            Point3::new(x_min, y_min, z_min),
            Point3::new(x_max, y_max, z_max),
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

    fn bounding_box(&self) -> AABB {
        self.transform_bounding_box(self.shape.bounding_box())
    }

    fn hit(&self, ray: &Ray) -> bool {
        let inv_ray = self.transformation.apply_inverse(ray);
        self.shape.hit(&inv_ray)
    }
}
