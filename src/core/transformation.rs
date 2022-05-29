use nalgebra::{vector, Point3, Vector3};
use nalgebra_glm as glm;
use nalgebra_glm::{TMat4, TVec3};

use crate::core::{Aabb, Normal3, Ray};
use crate::Float;

pub struct Transformation {
    mat: TMat4<Float>,
    inv: TMat4<Float>,
}

impl Default for Transformation {
    fn default() -> Self {
        let mat = TMat4::default();
        let inv = TMat4::default();

        Self { mat, inv }
    }
}

impl Transformation {
    pub fn translate(x: Float, y: Float, z: Float) -> Self {
        let mat = glm::translation(&vector![x, y, z]);
        let inv = glm::translation(&vector![-x, -y, -z]);

        Self { mat, inv }
    }

    pub fn rotate(angle: Float, axis: &TVec3<Float>) -> Self {
        let mat = glm::rotation(angle, axis);
        let inv = glm::transpose(&mat);

        Self { mat, inv }
    }

    pub fn rotate_x(angle: Float) -> Self {
        Self::rotate(angle, &vector![1., 0., 0.])
    }

    pub fn rotate_y(angle: Float) -> Self {
        Self::rotate(angle, &vector![0., 1., 0.])
    }

    pub fn rotate_z(angle: Float) -> Self {
        Self::rotate(angle, &vector![0., 0., 1.])
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Self {
        let mat = glm::scaling(&vector![x, y, z]);
        let inv = glm::scaling(&vector![1. / x, 1. / y, 1. / z]);

        Self { mat, inv }
    }

    #[must_use]
    pub fn invert(&self) -> Self {
        let mat = self.inv;
        let inv = self.mat;

        Self { mat, inv }
    }

    #[must_use]
    pub fn then(&self, other: &Self) -> Self {
        let mat = other.mat * self.mat;
        let inv = self.inv * other.mat;

        Self { mat, inv }
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
        t.mat.transform_point(self)
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        t.inv.transform_point(self)
    }
}

impl Transformable for Vector3<Float> {
    fn transform(&self, t: &Transformation) -> Self {
        t.mat.transform_vector(self)
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        t.inv.transform_vector(self)
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
        Normal3(glm::transpose(&t.inv).transform_vector(&self.0))
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        Normal3(glm::transpose(&t.mat).transform_vector(&self.0))
    }
}

impl Transformable for Aabb {
    fn transform(&self, t: &Transformation) -> Self {
        self.vertices()
            .iter()
            .map(|p| t.mat.transform_point(p))
            .fold(Aabb::default(), |bbox, p| bbox.union(p))
    }

    fn transform_inverse(&self, t: &Transformation) -> Self {
        self.vertices()
            .iter()
            .map(|p| t.inv.transform_point(p))
            .fold(Aabb::default(), |bbox, p| bbox.union(p))
    }
}
