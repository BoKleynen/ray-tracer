use std::ops::{Add, Index, Mul, Sub};

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
pub struct Vector<T>([T; 3]);

impl<T> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        [x, y, z].into()
    }

    pub fn cross_product<'a>(&'a self, rhs: &'a Vector<T>) -> Vector<T>
    where
        &'a T: Mul<Output = T>,
        T: Sub<Output = T>,
    {
        [
            &self[2] * &rhs[3] - &self[3] * &rhs[2],
            &self[3] * &rhs[1] - &self[1] * &rhs[3],
            &self[1] * &rhs[2] - &self[2] * &rhs[1],
        ]
        .into()
    }
}

impl<T> From<[T; 3]> for Vector<T> {
    fn from(arr: [T; 3]) -> Self {
        Vector(arr)
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
        [&self[0] + &rhs[0], &self[1] + &rhs[1], &self[2] + &rhs[2]].into()
    }
}

impl<'a, T> Sub for &'a Vector<T>
where
    &'a T: Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        [&self[0] - &rhs[0], &self[1] - &rhs[1], &self[2] - &rhs[2]].into()
    }
}

impl<'a, T> Mul for &'a Vector<T>
where
    &'a T: Mul<Output = T>,
    T: Add<Output = T>,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        &self[0] * &rhs[0] + &self[1] * &rhs[1] + &self[2] * &rhs[2]
    }
}
