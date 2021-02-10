use crate::math::homogeneous::{Point, Ray, Transformation};
use crate::shape::Intersectable;

/// A three-dimensional cuboid, bounded by the origin and the oposite corner.
#[derive(Debug)]
pub struct Cuboid {
    transformation: Transformation,
    corner: Point,
}

impl Intersectable for Cuboid {
    fn intersect(&self, ray: &Ray) -> bool {
        let inv_ray = self.transformation.apply_inverse(ray);

        let ox = inv_ray.origin().x();
        let oy = inv_ray.origin().y();
        let oz = inv_ray.origin().z();

        let a = 1.0 / inv_ray.direction().x();
        let (tx_min, tx_max) = if a >= 0.0 {
            ((0.0 - ox) * a, (self.corner.x() - ox) * a)
        } else {
            ((self.corner.x() - ox) * a, (0.0 - ox) * a)
        };

        let b = 1.0 / inv_ray.direction().y();
        let (ty_min, ty_max) = if b >= 0.0 {
            ((0.0 - oy) * b, (self.corner.y() - ox) * b)
        } else {
            ((self.corner.y() - oy) * b, (0.0 - oy) * b)
        };

        let c = 1.0 / inv_ray.direction().z();
        let (tz_min, tz_max) = if c >= 0.0 {
            ((0.0 - oz) * c, (self.corner.z() - oz) * c)
        } else {
            ((self.corner.z() - oz) * c, (0.0 - oz) * c)
        };

        // find largest entering t value
        let t0 = tx_min.max(ty_min).max(tz_min);

        // find smallest exiting t value
        let t1 = tx_max.min(ty_max).min(tz_max);

        t0 < t1 && t1 > f64::EPSILON
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
            corner: self.bound,
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
