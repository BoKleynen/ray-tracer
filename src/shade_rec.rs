use crate::film::RGB;
use crate::world::World;
use nalgebra::{Point3, Vector3};
use crate::math::Ray;

pub struct ShadeRec<'a> {
    pub hit_point: Point3<f64>,
    pub local_hit_point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub color: RGB,
    pub ray: Ray,
    pub depth: u32,
    pub direction: Vector3<f64>,
    pub world: &'a World,
}
