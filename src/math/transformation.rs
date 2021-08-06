use crate::{Point3, Vector};
use nalgebra as na;
use nalgebra::{Affine3, Matrix4, Rotation3, Translation3, Unit};

#[derive(Debug)]
pub struct Transformation {
    matrix: Affine3<f64>,
    inverse: Affine3<f64>,
}

impl Transformation {
    pub fn new(matrix: Affine3<f64>, inverse: Affine3<f64>) -> Self {
        Self { matrix, inverse }
    }

    pub fn invert(self) -> Self {
        Self::new(self.inverse, self.matrix)
    }

    pub fn then(&self, other: &Self) -> Self {
        let matrix = other.matrix * self.matrix;
        let inverse = self.inverse * other.inverse;

        Self::new(matrix, inverse)
    }

    pub fn apply<U: Transformable>(&self, obj: &U) -> U {
        obj.transform(&self.matrix)
    }

    pub fn apply_inverse<U: Transformable>(&self, obj: &U) -> U {
        obj.transform(&self.inverse)
    }

    pub fn identity() -> Self {
        let matrix = Affine3::identity();
        let inverse = Affine3::identity();

        Self { matrix, inverse }
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Self {
        let matrix = na::convert(Translation3::from(Vector::new(x, y, z)));
        let inverse = na::convert(Translation3::from(Vector::new(-x, -y, -z)));

        Self { matrix, inverse }
    }

    pub fn rotate(axis: &Unit<Vector>, angle: f64) -> Self {
        let matrix = Rotation3::from_axis_angle(axis, angle);

        let inverse = na::convert(matrix.inverse());
        let matrix = na::convert(matrix);

        Self { matrix, inverse }
    }

    pub fn rotate_x(angle: f64) -> Transformation {
        let angle = angle.to_radians();
        let x = Unit::new_unchecked(Vector::new(1., 0., 0.));

        Self::rotate(&x, angle)
    }

    pub fn rotate_y(angle: f64) -> Transformation {
        let angle = angle.to_radians();
        let y = Unit::new_unchecked(Vector::new(0., 1., 0.));

        Self::rotate(&y, angle)
    }

    pub fn rotate_z(angle: f64) -> Transformation {
        let angle = angle.to_radians();
        let z = Unit::new_unchecked(Vector::new(0., 0., 1.));

        Self::rotate(&z, angle)
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        let matrix = na::convert_unchecked(Matrix4::new_nonuniform_scaling(&Vector::new(x, y, z)));
        let inverse = na::convert_unchecked(Matrix4::new_nonuniform_scaling(&Vector::new(
            1. / x,
            1. / y,
            1. / z,
        )));

        Self { matrix, inverse }
    }

    pub fn matrix(&self) -> &Affine3<f64> {
        &self.matrix
    }

    pub fn inverse(&self) -> &Affine3<f64> {
        &self.inverse
    }
}

pub trait Transformable {
    fn transform(&self, matrix: &Affine3<f64>) -> Self;
}

impl Transformable for Vector {
    fn transform(&self, matrix: &Affine3<f64>) -> Self {
        matrix * self
    }
}

impl Transformable for Point3 {
    fn transform(&self, matrix: &Affine3<f64>) -> Self {
        matrix * self
    }
}
