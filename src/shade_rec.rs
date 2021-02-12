use crate::film::RGB;
use crate::world::World;
use nalgebra::{Point3, Vector3};

pub struct ShadeRec<'a> {
    pub hit_an_object: bool,
    pub hit_point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub color: RGB,
    world: &'a World,
}

impl<'a> ShadeRec<'a> {
    pub fn new(world: &'a World) -> Self {
        Self {
            hit_an_object: false,
            hit_point: Point3::origin(),
            normal: Vector3::default(),
            color: RGB::black(),
            world,
        }
    }
}
