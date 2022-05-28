use crate::core::{Aabb, Normal3, Ray};
use crate::Float;
use nalgebra as na;
use nalgebra::{
    vector, Affine3, Matrix4, Point3, Rotation3, Scale3, Similarity3, Translation3, Unit, Vector3,
};

#[derive(Debug, Clone)]
pub struct Transformation {
    mat: Affine3<Float>,
    inv: Affine3<Float>,
}

impl Default for Transformation {
    fn default() -> Self {
        let mat = Affine3::default();
        let inv = Affine3::default();

        Self { mat, inv }
    }
}

impl Transformation {
    pub fn new(mat: Affine3<Float>) -> Self {
        let inv = mat.inverse();
        Self { mat, inv }
    }

    pub fn translate(x: Float, y: Float, z: Float) -> Self {
        let translation = Translation3::from(vector![x, y, z]);
        let mat = na::convert(translation);
        let inv = na::convert(translation.inverse());

        Self { mat, inv }
    }

    pub fn rotate(axis: &Unit<Vector3<Float>>, angle: Float) -> Self {
        let rot = Rotation3::from_axis_angle(axis, angle);
        let mat = na::convert(rot);
        let inv = na::convert(rot.inverse());

        Self { mat, inv }
    }

    pub fn rotate_x(angle: Float) -> Transformation {
        let x = Unit::new_unchecked(vector![1., 0., 0.]);
        Self::rotate(&x, angle)
    }

    pub fn rotate_y(angle: Float) -> Transformation {
        let y = Unit::new_unchecked(vector![0., 1., 0.]);
        Self::rotate(&y, angle)
    }

    pub fn rotate_z(angle: Float) -> Transformation {
        let z = Unit::new_unchecked(vector![0., 0., 1.]);
        Self::rotate(&z, angle)
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Self {
        debug_assert_ne!(x, 0.);
        debug_assert_ne!(y, 0.);
        debug_assert_ne!(z, 0.);

        let scale = Scale3::new(x, y, z);
        let mat = na::convert(scale);
        // SAFETY: x, y and z are guaranteed to be non-zero
        let inv = na::convert(unsafe { scale.inverse_unchecked() });

        Self { mat, inv }
    }

    pub fn matrix(&self) -> &Affine3<Float> {
        &self.mat
    }

    pub fn inverse(&self) -> &Affine3<Float> {
        &self.inv
    }

    #[must_use]
    pub fn invert(self) -> Self {
        Self {
            mat: self.inv,
            inv: self.mat,
        }
    }

    #[must_use]
    pub fn then(&self, other: &Self) -> Self {
        let matrix = other.mat * self.mat;
        let inverse = self.inv * other.inv;

        Self {
            mat: matrix,
            inv: inverse,
        }
    }

    #[inline]
    pub fn apply<T: Transformable>(&self, obj: &T) -> T {
        obj.transform(self)
    }

    #[inline]
    pub fn apply_inverse<T: Transformable>(&self, obj: &T) -> T {
        obj.transform_inverse(self)
    }
}

pub trait Transformable {
    fn transform(&self, t: &Transformation) -> Self;
    fn transform_inverse(&self, t: &Transformation) -> Self;
}

impl Transformable for Point3<Float> {
    fn transform(&self, t: &Transformation) -> Self {
        t.mat * self
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        t.inv * self
    }
}

impl Transformable for Vector3<Float> {
    fn transform(&self, t: &Transformation) -> Self {
        t.mat * self
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        t.inv * self
    }
}

impl Transformable for Ray {
    fn transform(&self, t: &Transformation) -> Self {
        let o = t.apply(&self.o);
        let d = t.apply(&self.d);

        Ray { o, d, ..*self }
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        let o = t.apply_inverse(&self.o);
        let d = t.apply_inverse(&self.d);

        Ray { o, d, ..*self }
    }
}

impl Transformable for Normal3<Float> {
    fn transform(&self, t: &Transformation) -> Self {
        Normal3(t.inv.matrix().transpose().transform_vector(&self.0))
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        Normal3(t.mat.matrix().transpose().transform_vector(&self.0))
    }
}

impl Transformable for Aabb {
    fn transform(&self, t: &Transformation) -> Self {
        self.vertices()
            .iter()
            .map(|p| t.mat * p)
            .fold(Aabb::default(), |bbox, p| bbox.union(p))
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        self.vertices()
            .iter()
            .map(|p| t.inv * p)
            .fold(Aabb::default(), |bbox, p| bbox.union(p))
    }
}
