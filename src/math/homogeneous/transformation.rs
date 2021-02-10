use crate::math::{Matrix, SquareMatrix};

pub type TransformationMatrix = SquareMatrix<f64, 4>;

#[derive(Debug)]
pub struct Transformation {
    matrix: TransformationMatrix,
    inverse: TransformationMatrix,
}

impl Transformation {
    pub fn new(matrix: TransformationMatrix, inverse: TransformationMatrix) -> Self {
        Self { matrix, inverse }
    }

    pub fn identity() -> Self {
        let matrix = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inverse = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { matrix, inverse }
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Self {
        let matrix = Matrix::from([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inverse = Matrix::from([
            [1.0, 0.0, 0.0, -x],
            [0.0, 1.0, 0.0, -y],
            [0.0, 0.0, 1.0, -z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { matrix, inverse }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        let matrix = Matrix::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inverse = Matrix::from([
            [1.0 / x, 0.0, 0.0, 0.0],
            [0.0, 1.0 / y, 0.0, 0.0],
            [0.0, 0.0, 1.0 / z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        Self { matrix, inverse }
    }

    pub fn invert(self) -> Self {
        Self::new(self.inverse, self.matrix)
    }

    pub fn append<'a>(&'a self, other: &'a Self) -> Self {
        let matrix = &self.matrix * &other.matrix;
        let inverse = &self.inverse * &other.inverse;

        Self::new(matrix, inverse)
    }

    pub fn apply<U: Transformable>(&self, obj: &U) -> U {
        obj.transform(&self.matrix)
    }

    pub fn apply_inverse<U: Transformable>(&self, obj: &U) -> U {
        obj.transform(&self.inverse)
    }
}

pub trait Transformable {
    fn transform(&self, matrix: &TransformationMatrix) -> Self;
}
