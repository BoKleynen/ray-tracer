use crate::math::Vector;
use std::cmp::PartialEq;
use std::ops::{Add, Index, Sub};

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
pub struct Point<T>(Vector<T, 3>);

impl<T> From<[T; 3]> for Point<T> {
    fn from(arr: [T; 3]) -> Self {
        Point(Vector::from(arr))
    }
}

impl<T> Index<usize> for Point<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, T> Add<&'a Vector<T, 3>> for &'a Point<T>
where
    &'a Vector<T, 3>: Add<Output = Vector<T, 3>>,
{
    type Output = Point<T>;

    fn add(self, rhs: &'a Vector<T, 3>) -> Self::Output {
        Point(&self.0 + rhs)
    }
}

impl<'a, T> Sub for &'a Point<T>
where
    &'a Vector<T, 3>: Sub<Output = Vector<T, 3>>,
{
    type Output = Vector<T, 3>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self.0 - &rhs.0
    }
}
