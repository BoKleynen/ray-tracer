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
        let mut t_min = f64::INFINITY;
        let mut color = RGB::black();

        self.world.shapes().iter().for_each(|shape| {
            if let Some(hit) = shape.intersect(ray) {
                if hit.t < t_min {
                    t_min = hit.t;
                    color = shape.color();
                }
            }
        });

        color
    }
}
