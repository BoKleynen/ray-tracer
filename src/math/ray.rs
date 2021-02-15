use super::Transformable;
use nalgebra::{Affine3, Point3, Vector3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3<f64> {
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
