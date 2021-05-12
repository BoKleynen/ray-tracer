use nalgebra::Affine3;

use super::Transformable;
use crate::{Point3, Vector};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}

impl Transformable for Ray {
    fn transform(&self, matrix: &Affine3<f64>) -> Self {
        let origin = self.origin.transform(matrix);
        let direction = self.direction.transform(matrix);

        Self { origin, direction }
    }
}
