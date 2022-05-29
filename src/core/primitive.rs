use std::sync::Arc;

use crate::core::{Aabb, AreaLight, Bounded, Material, Shape};

pub trait Primitive: Bounded {}

pub struct GeometricPrimitive {
    shape: Arc<dyn Shape>,
    material: Arc<dyn Material>,
    area_light: Option<Arc<dyn AreaLight>>,
}

impl Bounded for GeometricPrimitive {
    fn bbox(&self) -> Aabb {
        self.shape.bbox()
    }
}
