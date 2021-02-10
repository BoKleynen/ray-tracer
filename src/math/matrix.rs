use super::Vector;
use crate::const_iter::{self, Iterator};
use std::mem::{self, MaybeUninit};
use std::ops::{Add, AddAssign, Index, Mul, Sub};

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize>(Vector<Vector<T, N>, M>);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    fn rows(&self) -> RowIterator<T, M, N> {
        RowIterator::new(self)
    }
}

impl<T, const M: usize, const N: usize> const_iter::FromIterator<Vector<T, N>, M>
    for Matrix<T, M, N>
{
    fn from_const_iter<I: Iterator<M, Item = Vector<T, N>>>(iter: I) -> Self {
        Matrix(Vector::from_const_iter(iter))
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from(arr: [[T; N]; M]) -> Self {
        let p = arr.as_ptr() as *const Matrix<T, M, N>;
        mem::forget(arr);
        unsafe { p.read() }
    }
}

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = Vector<T, N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, T, const M: usize, const N: usize> Add for &'a Matrix<T, M, N>
where
    &'a T: Add<Output = T>,
{
    type Output = Matrix<T, M, N>;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix(&self.0 + &rhs.0)
    }
}

impl<'a, T, const M: usize, const N: usize> Sub for &'a Matrix<T, M, N>
where
    &'a T: Sub<Output = T>,
{
    type Output = Matrix<T, M, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix(&self.0 - &rhs.0)
    }
}

impl<T, const M: usize, const N: usize> Mul<&Vector<T, N>> for &Matrix<T, M, N>
where
    for<'a> &'a Vector<T, N>: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn mul(self, rhs: &Vector<T, N>) -> Self::Output {
        self.rows().map(|a| a * rhs).collect()
    }
}

impl<T, const M: usize, const N: usize, const P: usize> Mul<&Matrix<T, M, N>> for &Matrix<T, N, P>
where
    for<'a> &'a T: Mul<Output = T>,
    T: AddAssign,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: &Matrix<T, M, N>) -> Self::Output {
        let mut result: MaybeUninit<Matrix<T, M, P>> = MaybeUninit::uninit();
        let base = result.as_mut_ptr() as *mut T;

        for i in 0..M {
            for j in 0..P {
                let mut elem = &self[i][0] * &rhs[0][j];
                for k in 1..N {
                    elem += &self[i][k] * &rhs[k][j]
                }
                unsafe {
                    base.add(P * i + j).write(elem);
                }
            }
        }

        unsafe { result.assume_init() }
    }
}

pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;

struct RowIterator<'a, T, const M: usize, const N: usize> {
    idx: usize,
    m: &'a Matrix<T, M, N>,
}

impl<'a, T, const M: usize, const N: usize> RowIterator<'a, T, M, N> {
    fn new(m: &'a Matrix<T, M, N>) -> Self {
        Self { idx: 0, m }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator<M> for RowIterator<'a, T, M, N> {
    type Item = &'a Vector<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == M {
            return None;
        }

        let i = self.idx;
        self.idx += 1;
        Some(&self.m[i])
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn add_matrix() {
        let a = Matrix::from([[1, 2], [3, 4]]);
        let b = Matrix::from([[5, 6], [7, 8]]);
        let c = &a + &b;

        assert_eq!(c, Matrix::from([[6, 8], [10, 12]]))
    }
}
