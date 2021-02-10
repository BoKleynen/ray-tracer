use super::{Point, Vector};

#[derive(Debug, Eq, PartialEq)]
pub struct Ray<T> {
    origin: Point<T>,
    direction: Vector<T, 3>,
}

impl<T> Ray<T> {
    pub fn new(origin: Point<T>, direction: Vector<T, 3>) -> Self {
        Self { origin, direction }
    }
}
