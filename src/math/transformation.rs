use nalgebra::{Point3, Vector3, Translation3, Affine3, Matrix4};
use nalgebra as na;

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

    pub fn append(&self, other: &Self) -> Self {
        let matrix = &self.matrix * &other.matrix;
        let inverse = &other.inverse * &self.inverse;

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
        let matrix = na::convert(Translation3::from(Vector3::new(x, y, z)));
        let inverse = na::convert(Translation3::from(Vector3::new(-x, -y, -z)));

        Self { matrix, inverse }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        let matrix = na::convert_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z)));
        let inverse = na::convert_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(1.0/x, 1.0/y, 1.0/z)));

        Self { matrix, inverse }
    }
}

pub trait Transformable {
    fn transform(&self, matrix: &Affine3<f64>) -> Self;
}

impl Transformable for Vector3<f64> {
    fn transform(&self, matrix: &Affine3<f64>) -> Self {
        matrix * self
    }
}

impl Transformable for Point3<f64> {
    fn transform(&self, matrix: &Affine3<f64>) -> Self {
        matrix * self
    }
}
