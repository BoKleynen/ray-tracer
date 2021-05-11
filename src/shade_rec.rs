use crate::material::Material;
use crate::world::World;
use crate::{Point, Vector};

pub struct ShadeRec<'a> {
    pub hit_point: Point,
    pub local_hit_point: Point,
    pub normal: Vector,
    pub material: Material,
    pub depth: u32,
    pub direction: Vector,
    pub world: &'a World,
}
