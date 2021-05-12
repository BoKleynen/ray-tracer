use crate::material::Material;
use crate::world::World;
use crate::{Point2, Point3, Vector};

pub struct ShadeRec<'a> {
    pub hit_point: Point3,
    pub local_hit_point: Point3,
    pub uv: Point2,
    pub normal: Vector,
    pub material: Material,
    pub depth: u32,
    pub direction: Vector,
    pub world: &'a World,
}
