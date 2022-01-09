use std::ops::Deref;

mod aabb;
mod ray;

pub use aabb::Aabb;
pub use ray::Ray;

pub trait Bounded {
    fn bbox(&self) -> Aabb;
}

impl<S, T> Bounded for T
where
    S: Bounded,
    T: Deref<Target = S>,
{
    fn bbox(&self) -> Aabb {
        (**self).bbox()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const fn all() -> [Self; 3] {
        [Axis::X, Axis::Y, Axis::Z]
    }
}
