use crate::film::RGB;
use crate::math::{Ray, Transformation};
use crate::shape::Shape;
use nalgebra::Point3;

/// A three-dimensional cuboid bounded by a corner and it's mirror with respect
/// to the origin.
#[derive(Debug)]
pub struct Cuboid {
    transformation: Transformation,
    corner: Point3<f64>,
    color: RGB,
}

enum CuboidFace {
    Left,
    Bottom,
    Back,
    Right,
    Top,
    Front,
}

impl Cuboid {
    pub fn new(corner: Point3<f64>, transformation: Transformation, color: RGB) -> Self {
        Self {
            transformation,
            corner,
            color,
        }
    }
}

impl Shape for Cuboid {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let inv_ray = self.transformation.apply_inverse(ray);

        let ox = inv_ray.origin().x;
        let oy = inv_ray.origin().y;
        let oz = inv_ray.origin().z;

        let dx = inv_ray.direction().x;
        let dy = inv_ray.direction().y;
        let dz = inv_ray.direction().z;

        let a = 1.0 / dx;
        let (tx_min, tx_max) = if a >= 0.0 {
            ((-self.corner.x - ox) * a, (self.corner.x - ox) * a)
        } else {
            ((self.corner.x - ox) * a, (-self.corner.x - ox) * a)
        };

        let b = 1.0 / dy;
        let (ty_min, ty_max) = if b >= 0.0 {
            ((-self.corner.y - oy) * b, (self.corner.y - ox) * b)
        } else {
            ((self.corner.y - oy) * b, (-self.corner.y - oy) * b)
        };

        let c = 1.0 / dz;
        let (tz_min, tz_max) = if c >= 0.0 {
            ((-self.corner.z - oz) * c, (self.corner.z - oz) * c)
        } else {
            ((self.corner.z - oz) * c, (-self.corner.z - oz) * c)
        };

        // find largest entering t value
        let t0 = tx_min.max(ty_min).max(tz_min);
        // let (mut t0, mut face_in) = if tx_min > ty_min {
        //     (tx_min, if a >= 0.0 { CuboidFace::Left } else { CuboidFace::Right })
        // } else {
        //     (ty_max, if b >= 0.0 { CuboidFace::Bottom } else { CuboidFace::Top })
        // };
        //
        // if tz_min > t0 {
        //     t0 = tz_min;
        //     face_in = if c >= 0.0 { CuboidFace::Back } else { CuboidFace::Front }
        // }

        // find smallest exiting t value
        let t1 = tx_max.min(ty_max).min(tz_max);
        // let (mut t1, mut face_out) = if tx_max < ty_max {
        //     (tx_max, if a >= 0.0 { CuboidFace::Right } else { CuboidFace::Left })
        // } else {
        //     (ty_max, if b >= 0.0 { CuboidFace::Top } else { CuboidFace::Bottom })
        // };
        //
        // if tz_max < t1 {
        //     t1 = tz_max;
        //     face_out = if c >= 0.0 { CuboidFace::Front } else { CuboidFace::Back }
        // }

        if t0 < t1 && t1 > f64::EPSILON {
            if t0 > f64::EPSILON {
                Some(t0)
            } else {
                Some(t1)
            }
        } else {
            None
        }
    }

    fn color(&self) -> RGB {
        self.color
    }
}

struct CuboidBuilder {
    transformation: Option<Transformation>,
    bound: Point3<f64>,
    color: Option<RGB>,
}

impl CuboidBuilder {
    fn new(bound: Point3<f64>) -> Self {
        Self {
            transformation: None,
            bound,
            color: None,
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
            color: self.color.unwrap_or(RGB::new(0.0, 0.0, 1.0)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shape::Shape;
    use nalgebra::{Point3, Vector3};

    #[test]
    fn non_intersecting_ray() {
        let cuboid = CuboidBuilder::new(Point3::new(1.0, 1.0, 1.0)).build();
        let ray = Ray::new(Point3::new(2.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));

        assert_eq!(cuboid.intersect(&ray), false)
    }

    #[test]
    fn intersecting_ray() {
        let cuboid = CuboidBuilder::new(Point3::new(1.0, 1.0, 1.0)).build();
        let ray = Ray::new(Point3::new(0.5, 0.5, 0.5), Vector3::new(-1.0, 0.0, 0.0));

        assert_eq!(cuboid.intersect(&ray), true)
    }

    #[test]
    fn transformed_intersecting_ray() {
        let t = Transformation::scale(10.0, 10.0, 10.0);
        let cuboid = CuboidBuilder::new(Point3::new(1.0, 1.0, 1.0))
            .transformation(t)
            .build();
        let ray = Ray::new(Point3::new(2.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));

        assert_eq!(cuboid.intersect(&ray), true)
    }
}
