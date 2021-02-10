use crate::math::SquareMatrix;

pub type TransformationMatrix = SquareMatrix<f64, 4>;

pub struct Transformation {
    matrix: TransformationMatrix,
    inverse: TransformationMatrix,
}

impl Transformation {
    pub fn new(matrix: TransformationMatrix, inverse: TransformationMatrix) -> Self {
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
