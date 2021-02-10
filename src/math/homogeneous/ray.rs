use super::transformation::TransformationMatrix;
use super::{Point, Transformable, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}

impl Transformable for Ray {
    fn transform(&self, matrix: &TransformationMatrix) -> Self {
        let origin = self.origin.transform(matrix);
        let direction = self.direction.transform(matrix);

        Self { origin, direction }
    }
}
