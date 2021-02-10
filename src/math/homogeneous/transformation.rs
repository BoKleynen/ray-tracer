use crate::math::SquareMatrix;
use std::ops::Mul;

pub struct Transformation<T> {
    matrix: SquareMatrix<T, 4>,
    inverse: SquareMatrix<T, 4>,
}

impl<T> Transformation<T> {
    pub fn new(matrix: SquareMatrix<T, 4>, inverse: SquareMatrix<T, 4>) -> Self {
        Self { matrix, inverse }
    }

    pub fn invert(self) -> Self {
        Self::new(self.inverse, self.matrix)
    }

    pub fn append<'a>(&'a self, other: &'a Self) -> Self
    where
        &'a SquareMatrix<T, 4>: Mul<Output = SquareMatrix<T, 4>>,
    {
        let matrix = &self.matrix * &other.matrix;
        let inverse = &self.inverse * &other.inverse;

        Self::new(matrix, inverse)
    }

    pub fn apply<U: Transformable<T>>(&self, obj: U) -> U {
        obj.transform(&self.matrix)
    }

    pub fn apply_inverse<U: Transformable<T>>(&self, obj: U) -> U {
        obj.transform(&self.inverse)
    }
}

pub trait Transformable<T> {
    fn transform(&self, matrix: &SquareMatrix<T, 4>) -> Self;
}
