use super::Transformable;
use crate::math::{self, SquareMatrix};
use num_traits::Zero;
use std::ops::{Add, Index, Mul, Sub};

#[repr(transparent)]
pub struct Vector<T>(pub(crate) math::Vector<T, 4>);

impl<T: Copy + Zero> From<[T; 3]> for Vector<T> {
    fn from(arr: [T; 3]) -> Self {
        Vector(math::Vector::from([arr[0], arr[1], arr[2], T::zero()]))
    }
}

impl<T: Copy + Zero> From<math::Vector<T, 3>> for Vector<T> {
    fn from(v: math::Vector<T, 3>) -> Self {
        v.arr().into()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, T> Add for &'a Vector<T>
where
    &'a T: Add<Output = T>,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(&self.0 + &rhs.0)
    }
}

impl<'a, T> Sub for &'a Vector<T>
where
    &'a T: Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(&self.0 - &rhs.0)
    }
}

impl<T> Transformable<T> for Vector<T>
where
    for<'a> &'a SquareMatrix<T, 4>: Mul<&'a math::Vector<T, 4>, Output = math::Vector<T, 4>>,
{
    fn transform(&self, matrix: &SquareMatrix<T, 4>) -> Self {
        Vector(matrix * &self.0)
    }
}
