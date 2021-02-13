use crate::film::RGB;
use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape};
use nalgebra::{Point3, Vector3};

/// A three-dimensional cuboid bounded by a corner and it's mirror with respect
/// to the origin.
#[derive(Debug)]
pub struct Cuboid {
    transformation: Transformation,
    corner: Point3<f64>,
    material: Material,
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
    fn normal(self) -> Vector3<f64> {
        match self {
            CuboidFace::Left => Vector3::new(-1., 0., 0.),
            CuboidFace::Bottom => Vector3::new(0., -1., 0.),
            CuboidFace::Back => Vector3::new(0., 0., -1.),
            CuboidFace::Right => Vector3::new(1., 0., 0.),
            CuboidFace::Top => Vector3::new(0., 1., 0.),
            CuboidFace::Front => Vector3::new(0., 0., 1.),
        }
    }
}

impl Cuboid {
    pub fn new(corner: Point3<f64>, transformation: Transformation, material: Material) -> Self {
        Self {
            transformation,
            corner,
            material,
        }
    }
}

impl Shape for Cuboid {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
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
        let (mut t0, mut face_in) = if tx_min > ty_min {
            (
                tx_min,
                if a >= 0.0 {
                    CuboidFace::Left
                } else {
                    CuboidFace::Right
                },
            )
        } else {
            (
                ty_min,
                if b >= 0.0 {
                    CuboidFace::Bottom
                } else {
                    CuboidFace::Top
                },
            )
        };

        if tz_min > t0 {
            t0 = tz_min;
            face_in = if c >= 0.0 {
                CuboidFace::Back
            } else {
                CuboidFace::Front
            }
        }

        // find smallest exiting t value
        let (mut t1, mut face_out) = if tx_max < ty_max {
            (
                tx_max,
                if a >= 0.0 {
                    CuboidFace::Right
                } else {
                    CuboidFace::Left
                },
            )
        } else {
            (
                ty_max,
                if b >= 0.0 {
                    CuboidFace::Top
                } else {
                    CuboidFace::Bottom
                },
            )
        };

        if tz_max < t1 {
            t1 = tz_max;
            face_out = if c >= 0.0 {
                CuboidFace::Front
            } else {
                CuboidFace::Back
            }
        }

        if t0 < t1 && t1 > f64::EPSILON {
            if t0 > f64::EPSILON {
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

    fn material(&self) -> Material {
        self.material.clone()
    }
}
