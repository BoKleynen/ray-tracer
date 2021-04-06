use nalgebra::{Point3, Vector3};

use crate::material::Material;
use crate::world::World;

pub struct ShadeRec<'a> {
    pub hit_point: Point3<f64>,
    pub local_hit_point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
    pub depth: u32,
    pub direction: Vector3<f64>,
    pub world: &'a World,
}
