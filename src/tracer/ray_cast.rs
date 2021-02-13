use super::Tracer;
use crate::film::RGB;
use crate::math::Ray;
use crate::shade_rec::ShadeRec;
use crate::world::World;

pub struct RayCast<'a> {
    world: &'a World,
}

impl<'a> RayCast<'a> {
    pub fn new(world: &'a World) -> Self {
        Self { world }
    }
}

impl<'a> Tracer for RayCast<'a> {
    fn trace_ray(&self, ray: &Ray, depth: u32) -> RGB {
        match self.world.hit_objects(ray) {
            None => self.world.background_color(),
            Some(sr) => sr.material.shade(&sr, ray),
        }
    }
}
