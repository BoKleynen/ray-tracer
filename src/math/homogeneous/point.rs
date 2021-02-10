use super::{Transformable, Vector};
use crate::math::{self, SquareMatrix};
use num_traits::One;
use std::cmp::PartialEq;
use std::ops::{Add, Index, Mul, Sub};

/// A 3 dimensional point, represented in 4 dimensional homogeneous coordinates.
#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
pub struct Point<T>(math::Vector<T, 4>);

impl<T: Copy + One> From<[T; 3]> for Point<T> {
    fn from(arr: [T; 3]) -> Self {
        Point(math::Vector::from([arr[0], arr[1], arr[2], T::one()]))
    }
}

impl<T: Copy + One> From<math::Vector<T, 3>> for Point<T> {
    fn from(v: math::Vector<T, 3>) -> Self {
        v.arr().into()
    }
}

impl<T> Index<usize> for Point<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, T> Add<&'a Vector<T>> for &'a Point<T>
where
    &'a math::Vector<T, 4>: Add<Output = math::Vector<T, 4>>,
{
    type Output = Point<T>;

    fn add(self, rhs: &'a Vector<T>) -> Self::Output {
        Point(&self.0 + &rhs.0)
    }
}

impl<'a, T> Sub for &'a Point<T>
where
    &'a math::Vector<T, 4>: Sub<Output = math::Vector<T, 4>>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(&self.0 - &rhs.0)
    }
}

impl<T> Transformable<T> for Point<T>
where
    for<'a> &'a SquareMatrix<T, 4>: Mul<&'a math::Vector<T, 4>, Output = math::Vector<T, 4>>,
{
    fn transform(&self, matrix: &SquareMatrix<T, 4>) -> Self {
        // FIXME: multiply by 1.0 / w
        Point(matrix * &self.0)
    }
}
