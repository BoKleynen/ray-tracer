use crate::film::RGB;
use crate::math::Ray;
use crate::tracer::Tracer;
use crate::world::World;

pub struct MultipleObjects<'a> {
    world: &'a World,
}

impl<'a> MultipleObjects<'a> {
    pub fn new(world: &'a World) -> Self {
        Self { world }
    }
}

impl<'a> Tracer for MultipleObjects<'a> {
    fn trace_ray(&self, ray: &Ray, _depth: u32) -> RGB {
        match self.world.hit_objects(&ray) {
            None => RGB::black(),
            Some(sr) => sr.color,
        }
    }
}
