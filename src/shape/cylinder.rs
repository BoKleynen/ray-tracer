use crate::core::{Aabb, Bounded, Hit, Ray, Shape};
use crate::shape::Sphere;
use crate::{math, Float, Point3};

pub struct Cylinder {
    radius: Float,
    z_min: Float,
    z_max: Float,
}

impl Bounded for Cylinder {
    fn bbox(&self) -> Aabb {
        Aabb::new(
            Point3::new(-self.radius, -self.radius, self.z_min),
            Point3::new(self.radius, self.radius, self.z_max),
        )
    }
}

impl Shape for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let a = ray.direction.x * ray.direction.x + ray.direction.y + ray.direction.y;
        let b = 2. * (ray.direction.x * ray.origin.x + ray.direction.y * ray.origin.y);
        let c =
            ray.origin.x * ray.origin.x + ray.origin.y * ray.origin.y - self.radius * self.radius;

        let [t0, t1] = math::quadratic(a, b, c)?;
        if t1 < 0. {
            return None;
        }

        todo!()
    }

    fn area(&self) -> Float {
        todo!()
    }
}
