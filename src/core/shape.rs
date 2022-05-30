use std::ops::Deref;
use std::sync::Arc;

use crate::core::{Bounded, Ray, SurfaceInteraction};
use crate::Float;

pub trait Shape: Bounded {
    fn intersect(&self, ray: &Ray) -> Option<(Float, SurfaceInteraction)>;

    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn area(&self) -> Float;
}

// Implementing Shape for Deref<Target = impl Shape> results in lifetime problems,
// so we implement it on concrete types instead.
macro_rules! shape_impl {
    ($($t: ident) +) => {
        $(
            impl<T: Shape> Shape for $t<T> {
                #[inline]
                fn intersect(&self, ray: &Ray) -> Option<(Float, SurfaceInteraction)> {
                    self.deref().intersect(ray)
                }

                #[inline]
                fn intersects(&self, ray: &Ray) -> bool {
                    self.deref().intersects(ray)
                }

                #[inline]
                fn area(&self) -> Float {
                    self.deref().area()
                }
            }
        )+
    }
}

shape_impl! { Box Arc }
