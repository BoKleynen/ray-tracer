use crate::const_iter::{self, Iterator};
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::mem::MaybeUninit;
use std::ops::{Add, Index, Mul, Neg, Sub};

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    fn const_iter(&self) -> ConstIter<T, N> {
        ConstIter::new(self)
    }
}

impl<T> Vector<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        [x, y, z].into()
    }

    pub fn cross_product<'a>(&'a self, rhs: &'a Self) -> Self
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

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(arr: [T; N]) -> Self {
        Vector(arr)
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, T, const N: usize> Add for &'a Vector<T, N>
where
    &'a T: Add<Output = T>,
{
    type Output = Vector<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        self.const_iter()
            .zip(rhs.const_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}

impl<'a, T, const N: usize> Sub for &'a Vector<T, N>
where
    &'a T: Sub<Output = T>,
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.const_iter()
            .zip(rhs.const_iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

impl<'a, T, const N: usize> Mul for &'a Vector<T, N>
where
    T: const_iter::Sum<N>,
    &'a T: Mul<Output = T>,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.const_iter()
            .zip(rhs.const_iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<'a, T, const N: usize> Neg for &'a Vector<T, N>
where
    &'a T: Neg<Output = T>,
{
    type Output = Vector<T, N>;

    fn neg(self) -> Self::Output {
        self.const_iter().map(Neg::neg).collect()
    }
}

impl<T, const N: usize> Clone for Vector<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.const_iter().map(T::clone).collect()
    }
}

impl<T, const N: usize> const_iter::FromIterator<T, N> for Vector<T, N> {
    fn from_const_iter<I: const_iter::Iterator<N, Item = T>>(mut iter: I) -> Self {
        let mut result: MaybeUninit<Vector<T, N>> = MaybeUninit::uninit();
        let base = result.as_mut_ptr() as *mut T;

        for i in 0..N {
            match iter.next() {
                Some(elem) => unsafe {
                    base.add(i).write(elem);
                },
                None => unreachable!(),
            }
        }

        unsafe { result.assume_init() }
    }
}

impl<T: Display, const N: usize> Display for Vector<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..N {
            write!(f, "{}", self[i])?
        }

        Ok(())
    }
}

struct ConstIter<'a, T, const N: usize> {
    idx: usize,
    v: &'a Vector<T, N>,
}

impl<'a, T, const N: usize> ConstIter<'a, T, N> {
    fn new(v: &'a Vector<T, N>) -> Self {
        Self { idx: 0, v }
    }
}

impl<'a, T, const N: usize> Iterator<N> for ConstIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == N {
            return None;
        }

        let i = self.idx;
        self.idx += 1;
        Some(&self.v[i])
    }
}
