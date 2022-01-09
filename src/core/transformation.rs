use crate::core::Ray;
use crate::{Float, Normal3, Point3, Vector3};
use nalgebra as na;
use nalgebra::{Affine3, Matrix4, Rotation3, Translation3, Unit};

#[derive(Debug)]
pub struct Transformation {
    matrix: Affine3<Float>,
    inverse: Affine3<Float>,
}

impl Transformation {
    pub fn new(matrix: Affine3<Float>) -> Self {
        let inverse = matrix.inverse();
        Self { matrix, inverse }
    }

    pub fn identity() -> Self {
        let matrix = Affine3::identity();
        let inverse = Affine3::identity();

        Self { matrix, inverse }
    }

    pub fn translate(x: Float, y: Float, z: Float) -> Self {
        let matrix = na::convert(Translation3::from(Vector3::new(x, y, z)));
        let inverse = na::convert(Translation3::from(Vector3::new(-x, -y, -z)));

        Self { matrix, inverse }
    }

    pub fn rotate(axis: &Unit<Vector3>, angle: Float) -> Self {
        let matrix = Rotation3::from_axis_angle(axis, angle);

        let inverse = na::convert(matrix.inverse());
        let matrix = na::convert(matrix);

        Self { matrix, inverse }
    }

    pub fn rotate_x(angle: Float) -> Transformation {
        let angle = angle.to_radians();
        let x = Unit::new_unchecked(Vector3::new(1., 0., 0.));

        Self::rotate(&x, angle)
    }

    pub fn rotate_y(angle: Float) -> Transformation {
        let angle = angle.to_radians();
        let y = Unit::new_unchecked(Vector3::new(0., 1., 0.));

        Self::rotate(&y, angle)
    }

    pub fn rotate_z(angle: Float) -> Transformation {
        let angle = angle.to_radians();
        let z = Unit::new_unchecked(Vector3::new(0., 0., 1.));

        Self::rotate(&z, angle)
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Self {
        let matrix = na::convert_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z)));
        let inverse = na::convert_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(
            1. / x,
            1. / y,
            1. / z,
        )));

        Self { matrix, inverse }
    }

    pub fn matrix(&self) -> &Affine3<Float> {
        &self.matrix
    }

    pub fn inverse(&self) -> &Affine3<Float> {
        &self.inverse
    }

    #[must_use]
    pub fn invert(self) -> Self {
        Self {
            matrix: self.inverse,
            inverse: self.matrix,
        }
    }

    #[must_use]
    pub fn then(&self, other: &Self) -> Self {
        let matrix = other.matrix * self.matrix;
        let inverse = self.inverse * other.inverse;

        Self { matrix, inverse }
    }
}

pub trait Transform<T> {
    fn apply(&self, obj: &T) -> T;

    fn apply_inverse(&self, obj: &T) -> T;
}

impl Transform<Point3> for Transformation {
    fn apply(&self, p: &Point3) -> Point3 {
        self.matrix * p
    }

    fn apply_inverse(&self, p: &Point3) -> Point3 {
        self.inverse * p
    }
}

impl Transform<Vector3> for Transformation {
    fn apply(&self, v: &Vector3) -> Vector3 {
        self.matrix * v
    }

    fn apply_inverse(&self, v: &Vector3) -> Vector3 {
        self.inverse * v
    }
}

impl Transform<Normal3> for Transformation {
    fn apply(&self, n: &Normal3) -> Normal3 {
        Normal3(self.inverse.matrix().transpose().transform_vector(&n.0))
    }

    fn apply_inverse(&self, n: &Normal3) -> Normal3 {
        Normal3(self.matrix.matrix().transpose().transform_vector(&n.0))
    }
}

impl Transform<Ray> for Transformation {
    fn apply(&self, ray: &Ray) -> Ray {
        let o = self.apply(&ray.origin);
        let d = self.apply(&ray.direction);

        Ray {
            origin: o,
            direction: d,
        }
    }

    fn apply_inverse(&self, ray: &Ray) -> Ray {
        let o = self.apply_inverse(&ray.origin);
        let d = self.apply_inverse(&ray.direction);

        Ray {
            origin: o,
            direction: d,
        }
    }
}
