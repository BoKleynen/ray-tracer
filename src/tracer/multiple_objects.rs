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
    fn trace_ray(&self, ray: &Ray) -> RGB {
        let sr = self.world.hit_objects(ray);

        if sr.hit_an_object {
            sr.color
        } else {
            RGB::black()
        }
    }
}
