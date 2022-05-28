use crate::core::{Aabb, Bounded, Ray, Shape, SurfaceInteraction, Transformation};
use crate::Float;

pub struct Transformed<S> {
    shape: S,
    world_to_object: Transformation,
}

impl<S: Bounded> Bounded for Transformed<S> {
    fn bbox(&self) -> Aabb {
        let bbox = self.shape.bbox();
        self.world_to_object.apply(&bbox)
    }
}

impl<S: Shape> Shape for Transformed<S> {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let inv_ray = self.world_to_object.apply_inverse(ray);
        self.shape.intersect(&inv_ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let inv_ray = self.world_to_object.apply_inverse(ray);
        self.shape.intersects(&inv_ray)
    }

    fn area(&self) -> Float {
        todo!()
    }
}

impl<S> Transformed<S> {
    pub fn new(shape: S, world_to_object: Transformation) -> Self {
        Self {
            shape,
            world_to_object,
        }
    }
}
