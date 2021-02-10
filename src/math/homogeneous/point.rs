use super::transformation::TransformationMatrix;
use super::{Transformable, Vector};
use crate::math;
use std::cmp::PartialEq;
use std::ops::{Add, Index, Sub};

/// A 3 dimensional point, represented in 4 dimensional homogeneous coordinates.
#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct Point(math::Vector<f64, 4>);

impl From<[f64; 3]> for Point {
    fn from(arr: [f64; 3]) -> Self {
        Point(math::Vector::from([arr[0], arr[1], arr[2], 1.0]))
    }
}

impl From<math::Vector<f64, 3>> for Point {
    fn from(v: math::Vector<f64, 3>) -> Self {
        v.arr().into()
    }
}

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> Add<&'a Vector> for &'a Point {
    type Output = Point;

    fn add(self, rhs: &'a Vector) -> Self::Output {
        Point(&self.0 + &rhs.0)
    }
}

impl<'a> Sub for &'a Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(&self.0 - &rhs.0)
    }
}

impl Transformable for Point {
    fn transform(&self, matrix: &TransformationMatrix) -> Self {
        let p = matrix * &self.0;
        let inv_w = 1.0 / p[4];
        Point(&p * &inv_w)
    }
}
