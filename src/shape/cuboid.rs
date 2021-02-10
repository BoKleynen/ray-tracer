use crate::math::homogeneous::{Point, Ray, Transformation};
use crate::shape::Intersectable;

#[derive(Debug)]
pub struct Cuboid {
    transformation: Transformation,
    bound: Point,
}

impl Intersectable for Cuboid {
    fn intersect(&self, ray: &Ray) -> bool {
        let inv_ray = self.transformation.apply_inverse(ray);

        let a = 1.0 / inv_ray.direction()[0];
        let (tx_min, tx_max) = if a >= 0.0 {
            (-self.bound[0] * a, self.bound[0] * a)
        } else {
            (self.bound[0] * a, -self.bound[0] * a)
        };

        let b = 1.0 / inv_ray.direction()[1];
        let (ty_min, ty_max) = if b >= 0.0 {
            (-self.bound[1] * b, self.bound[1] * b)
        } else {
            (self.bound[1] * b, -self.bound[1] * b)
        };

        let c = 1.0 / inv_ray.direction()[2];
        let (tz_min, tz_max) = if c >= 0.0 {
            (-self.bound[2] * c, self.bound[2] * c)
        } else {
            (self.bound[2] * c, -self.bound[2] * c)
        };

        // find largest entering t value
        let t0 = tx_min.max(ty_min).max(tz_min);

        // find smallest exiting t value
        let t1 = tx_max.min(ty_max).min(tz_max);

        t0 < t1
    }
}

struct CuboidBuilder {
    transformation: Option<Transformation>,
    bound: Point,
}

impl CuboidBuilder {
    fn new(bound: Point) -> Self {
        Self {
            transformation: None,
            bound,
        }
    }

    fn transformation(mut self, transformation: Transformation) -> Self {
        self.transformation = Some(transformation);
        self
    }

    fn build(self) -> Cuboid {
        Cuboid {
            transformation: self.transformation.unwrap_or_else(Transformation::identity),
            bound: self.bound,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::math::homogeneous::{Ray, Vector};
    use crate::shape::Intersectable;
    use super::*;

    #[test]
    fn non_intersecting_ray() {
        let cuboid = CuboidBuilder::new(Point::new(1.0, 1.0, 1.0)).build();
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(cuboid.intersect(&ray), false)
    }

    #[test]
    fn intersecting_ray() {
        let cuboid = CuboidBuilder::new(Point::new(1.0, 1.0, 1.0)).build();
        let ray = Ray::new(Point::new(0.5, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0));

        assert_eq!(cuboid.intersect(&ray), true)
    }
}
