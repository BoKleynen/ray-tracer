use crate::math::Ray;
use crate::shape::{Aabb, Bounded, Hit, Shape};
use crate::{Point, Vector, K_EPSILON};

/// A three-dimensional cuboid bounded by a corner and it's mirror with respect
/// to the origin.
#[derive(Debug, Copy, Clone)]
pub struct Cuboid {
    corner: Point,
}

impl Cuboid {
    pub fn new(corner: Point) -> Self {
        Self { corner }
    }
}

impl Bounded for Cuboid {
    fn bbox(&self) -> Aabb {
        let (min_x, max_x) = if self.corner.x > 0. {
            (-self.corner.x, self.corner.x)
        } else {
            (self.corner.x, -self.corner.x)
        };
        let (min_y, max_y) = if self.corner.y > 0. {
            (-self.corner.y, self.corner.y)
        } else {
            (self.corner.y, -self.corner.y)
        };
        let (min_z, max_z) = if self.corner.z > 0. {
            (-self.corner.x, self.corner.z)
        } else {
            (self.corner.z, -self.corner.z)
        };

        Aabb::new(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }
}

impl Shape for Cuboid {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let ox = ray.origin().x;
        let oy = ray.origin().y;
        let oz = ray.origin().z;

        let dx = ray.direction().x;
        let dy = ray.direction().y;
        let dz = ray.direction().z;

        let a = 1. / dx;
        let (tx_min, tx_max) = if a >= 0. {
            ((-self.corner.x - ox) * a, (self.corner.x - ox) * a)
        } else {
            ((self.corner.x - ox) * a, (-self.corner.x - ox) * a)
        };

        let b = 1. / dy;
        let (ty_min, ty_max) = if b >= 0. {
            ((-self.corner.y - oy) * b, (self.corner.y - oy) * b)
        } else {
            ((self.corner.y - oy) * b, (-self.corner.y - oy) * b)
        };

        let c = 1. / dz;
        let (tz_min, tz_max) = if c >= 0. {
            ((-self.corner.z - oz) * c, (self.corner.z - oz) * c)
        } else {
            ((self.corner.z - oz) * c, (-self.corner.z - oz) * c)
        };

        // find largest entering t value
        let (mut t0, mut face_in) = if tx_min > ty_min {
            (
                tx_min,
                if a >= 0. {
                    CuboidFace::Left
                } else {
                    CuboidFace::Right
                },
            )
        } else {
            (
                ty_min,
                if b >= 0. {
                    CuboidFace::Bottom
                } else {
                    CuboidFace::Top
                },
            )
        };

        if tz_min > t0 {
            t0 = tz_min;
            face_in = if c >= 0. {
                CuboidFace::Back
            } else {
                CuboidFace::Front
            }
        }

        // find smallest exiting t value
        let (mut t1, mut face_out) = if tx_max < ty_max {
            (
                tx_max,
                if a >= 0. {
                    CuboidFace::Right
                } else {
                    CuboidFace::Left
                },
            )
        } else {
            (
                ty_max,
                if b >= 0. {
                    CuboidFace::Top
                } else {
                    CuboidFace::Bottom
                },
            )
        };

        if tz_max < t1 {
            t1 = tz_max;
            face_out = if c >= 0. {
                CuboidFace::Front
            } else {
                CuboidFace::Back
            }
        }

        if t0 < t1 && t1 > K_EPSILON {
            if t0 > K_EPSILON {
                Some(Hit {
                    t: t0,
                    normal: face_in.normal(),
                    local_hit_point: ray.origin() + t0 * ray.direction(),
                })
            } else {
                Some(Hit {
                    t: t1,
                    normal: face_out.normal(),
                    local_hit_point: ray.origin() + t1 * ray.direction(),
                })
            }
        } else {
            None
        }
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }
}

enum CuboidFace {
    Left,
    Bottom,
    Back,
    Right,
    Top,
    Front,
}

impl CuboidFace {
    fn normal(self) -> Vector {
        match self {
            CuboidFace::Left => Vector::new(-1., 0., 0.),
            CuboidFace::Bottom => Vector::new(0., -1., 0.),
            CuboidFace::Back => Vector::new(0., 0., -1.),
            CuboidFace::Right => Vector::new(1., 0., 0.),
            CuboidFace::Top => Vector::new(0., 1., 0.),
            CuboidFace::Front => Vector::new(0., 0., 1.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cuboid;
    use crate::math::Ray;
    use crate::shape::Shape;
    use nalgebra::{Point, Vector};

    #[test]
    fn obj_hit() {
        for ray in test_rays() {
            for cuboid in test_cuboids() {
                if cuboid.hit(&ray) {
                    assert!(cuboid.bbox().intersect(&ray));
                }
            }
        }
    }

    #[test]
    fn bounding_box_mis() {
        for ray in test_rays() {
            for cuboid in test_cuboids() {
                if !cuboid.bbox().intersect(&ray) {
                    assert!(!cuboid.hit(&ray));
                }
            }
        }
    }

    fn test_cuboids() -> Vec<Cuboid> {
        vec![
            Cuboid::new(Point::new(1., 1., 1.)),
            Cuboid::new(Point::new(-1., -1., -1.)),
        ]
    }

    fn test_rays() -> Vec<Ray> {
        vec![
            Ray::new(Point::new(0., 0., 0.), Vector::new(1., 0., 0.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(0., 1., 0.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(1., 1., 0.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(1., 1., 0.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(1., 0., 1.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(1., 0., 1.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(0., 1., 1.)),
            Ray::new(Point::new(0., 0., 0.), Vector::new(0., 1., 1.)),
        ]
    }
}
