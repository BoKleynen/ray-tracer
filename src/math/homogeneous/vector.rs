use super::Transformable;
use crate::math;
use crate::math::homogeneous::transformation::TransformationMatrix;
use std::ops::{Add, Index, Sub};

#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct Vector(pub(crate) math::Vector<f64, 4>);

impl From<[f64; 3]> for Vector {
    fn from(arr: [f64; 3]) -> Self {
        Vector(math::Vector::from([arr[0], arr[1], arr[2], 0.0]))
    }
}

impl From<math::Vector<f64, 3>> for Vector {
    fn from(v: math::Vector<f64, 3>) -> Self {
        v.arr().into()
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> Add for &'a Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(&self.0 + &rhs.0)
    }
}

impl<'a> Sub for &'a Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(&self.0 - &rhs.0)
    }
}

impl Transformable for Vector {
    fn transform(&self, matrix: &TransformationMatrix) -> Self {
        Vector(matrix * &self.0)
    }
}
