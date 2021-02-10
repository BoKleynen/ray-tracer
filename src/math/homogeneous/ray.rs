use super::{Vector, Point};
use crate::math::homogeneous::Transformable;
use crate::math::SquareMatrix;

pub struct Ray<T> {
    origin: Point<T>,
    direction: Vector<T>
}

impl<T> Transformable<T> for Ray<T>
where
    Point<T>: Transformable<T>,
    Vector<T>: Transformable<T>
{
    fn transform(&self, matrix: &SquareMatrix<T, 4>) -> Self {
        let origin = self.origin.transform(matrix);
        let direction = self.direction.transform(matrix);

        Self { origin, direction }
    }
}
