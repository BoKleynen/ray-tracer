use crate::world::World;
use crate::{Point2, Point3, Vector};
use crate::shape::GeometricObject;
use std::ptr::NonNull;

pub struct ShadeRec<'a> {
    pub hit_point: Point3,
    pub local_hit_point: Point3,
    pub uv: Point2,
    pub normal: Vector,
    pub direction: Vector,
    pub world: &'a World,
    pub(crate) shape: NonNull<GeometricObject>
}

impl<'a> ShadeRec<'a> {
    pub fn shape(&self) -> &GeometricObject {
        // safety: shapes are contained withing a world, so this reference will
        // at leas live as long as 'a.
        unsafe { self.shape.as_ref() }
    }
}
