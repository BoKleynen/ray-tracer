use crate::core::{Aabb, Bounded, Ray, Shape, SurfaceInteraction, Transformation};

pub struct Transformed<S> {
    shape: S,
    transform: Transformation,
}

impl<S: Bounded> Bounded for Transformed<S> {
    fn bbox(&self) -> Aabb {
        let bbox = self.shape.bbox();
        self.transform.apply(&bbox)
    }
}

impl<S: Shape> Shape for Transformed<S> {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let inv_ray = self.transform.apply_inverse(ray);
        self.shape.intersect(&inv_ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let inv_ray = self.transform.apply_inverse(ray);
        self.shape.intersects(&inv_ray)
    }
}
