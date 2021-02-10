use super::transformation::TransformationMatrix;
use super::{Point, Transformable, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Transformable for Ray {
    fn transform(&self, matrix: &TransformationMatrix) -> Self {
        let origin = self.origin.transform(matrix);
        let direction = self.direction.transform(matrix);

        Self { origin, direction }
    }
}
