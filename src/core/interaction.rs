use nalgebra::{Point2, Point3, Vector3};

use crate::core::{Normal3, Shape};
use crate::Float;

pub struct Shading {
    pub n: Normal3<Float>,
    pub dpdu: Vector3<Float>,
    pub dpdv: Vector3<Float>,
}

pub struct SurfaceInteraction<'a> {
    pub p: Point3<Float>,
    pub wo: Vector3<Float>,
    pub n: Normal3<Float>,
    pub uv: Point2<Float>,
    pub dpdu: Vector3<Float>,
    pub dpdv: Vector3<Float>,
    pub dndu: Normal3<Float>,
    pub dndv: Normal3<Float>,
    pub shape: &'a dyn Shape,
}
