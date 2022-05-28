use crate::core::{Normal3, Shape};
use crate::Float;
use nalgebra::{Point2, Point3, Vector3};

pub struct Shading {
    n: Normal3<Float>,
    dpdu: Vector3<Float>,
    dpdv: Vector3<Float>,
}

pub struct SurfaceInteraction<'a> {
    pub p: Point3<Float>,
    pub wo: Vector3<Float>,
    pub n: Normal3<Float>,
    pub uv: Point2<Float>,
    pub dpdu: Vector3<Float>,
    pub dpdv: Vector3<Float>,
    pub dndu: Vector3<Float>,
    pub dndv: Vector3<Float>,
    pub shape: &'a dyn Shape,
    pub shading: Shading,
}
