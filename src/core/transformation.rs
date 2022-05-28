use crate::core::Ray;
use crate::Float;
use nalgebra as na;
use nalgebra::{Affine3, Matrix4, Point3, Rotation3, Translation3, Unit, Vector3};

#[derive(Debug, Clone)]
pub struct Transformation {
    matrix: Affine3<Float>,
    inverse: Affine3<Float>,
}

impl Default for Transformation {
    fn default() -> Self {
        let matrix = Affine3::default();
        let inverse = Affine3::default();

        Self { matrix, inverse }
    }
}

impl Transformation {
    pub fn new(matrix: Affine3<Float>) -> Self {
        let inverse = matrix.inverse();
        Self { matrix, inverse }
    }

    pub fn translate(x: Float, y: Float, z: Float) -> Self {
        let matrix = na::convert(Translation3::from(Vector3::new(x, y, z)));
        let inverse = na::convert(Translation3::from(Vector3::new(-x, -y, -z)));

        Self { matrix, inverse }
    }

    pub fn rotate(axis: &Unit<Vector3<Float>>, angle: Float) -> Self {
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

    #[inline]
    fn apply<T: Transformable>(&self, obj: &T) -> T {
        obj.transform(self)
    }

    #[inline]
    fn apply_inverse<T: Transformable>(&self, obj: &T) -> T {
        obj.transform_inverse(self)
    }
}

pub trait Transformable {
    fn transform(&self, t: &Transformation) -> Self;
    fn transform_inverse(&self, t: &Transformation) -> Self;
}

impl Transformable for Point3<Float> {
    fn transform(&self, t: &Transformation) -> Self {
        t.matrix * self
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        t.inverse * self
    }
}

impl Transformable for Vector3<Float> {
    fn transform(&self, t: &Transformation) -> Self {
        t.matrix * self
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        t.inverse * self
    }
}

impl Transformable for Ray {
    fn transform(&self, t: &Transformation) -> Self {
        let origin = t.apply(&self.origin);
        let direction = t.apply(&self.direction);

        Ray { origin, direction }
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        let origin = t.apply_inverse(&self.origin);
        let direction = t.apply_inverse(&self.direction);

        Ray { origin, direction }
    }
}
