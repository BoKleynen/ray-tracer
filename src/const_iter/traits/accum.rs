use crate::const_iter::Iterator;
use std::num::Wrapping;
use std::ops::{Add, Mul};

pub trait Sum<const N: usize, A = Self>: Sized {
    /// Method which takes an iterator and generates `Self` from the elements by
    /// "summing up" the items.
    fn sum<I: Iterator<N, Item = A>>(iter: I) -> Self;
}

pub trait Product<const N: usize, A = Self>: Sized {
    /// Method which takes an iterator and generates `Self` from the elements by
    /// multiplying the items.
    fn product<I: Iterator<N, Item = A>>(iter: I) -> Self;
}

macro_rules! integer_const_sum_product {
    (@impls $zero:expr, $one:expr, $($a:ty)*) => ($(
        impl<const N: usize> Sum<N> for $a {
            fn sum<I: Iterator<N, Item=Self>>(iter: I) -> Self {
                iter.fold($zero, Add::add)
            }
        }

        impl<const N: usize> Product<N> for $a {
            fn product<I: Iterator<N, Item=Self>>(iter: I) -> Self {
                iter.fold($one, Mul::mul)
            }
        }

        impl<'a, const N: usize> Sum<N, &'a $a> for $a {
            fn sum<I: Iterator<N, Item=&'a Self>>(iter: I) -> Self {
                iter.fold($zero, Add::add)
            }
        }

        impl<'a, const N: usize> Product<N, &'a $a> for $a {
            fn product<I: Iterator<N, Item=&'a Self>>(iter: I) -> Self {
                iter.fold($one, Mul::mul)
            }
        }
    )*);
    ($($a:ty)*) => (
        integer_const_sum_product!(@impls 0, 1, $($a)*);
        integer_const_sum_product!(@impls Wrapping(0), Wrapping(1), $(Wrapping<$a>)*);
    );
}

macro_rules! float_const_sum_product {
    ($($a:ident)*) => ($(
        impl<const N: usize> Sum<N> for $a {
            fn sum<I: Iterator<N, Item=Self>>(iter: I) -> Self {
                iter.fold(0.0, Add::add)
            }
        }

        impl<const N: usize> Product<N> for $a {
            fn product<I: Iterator<N, Item=Self>>(iter: I) -> Self {
                iter.fold(1.0, Mul::mul)
            }
        }

        impl<'a, const N: usize> Sum<N, &'a $a> for $a {
            fn sum<I: Iterator<N, Item=&'a Self>>(iter: I) -> Self {
                iter.fold(0.0, Add::add)
            }
        }

        impl<'a, const N: usize> Product<N, &'a $a> for $a {
            fn product<I: Iterator<N, Item=&'a Self>>(iter: I) -> Self {
                iter.fold(1.0, Mul::mul)
            }
        }
    )*)
}

integer_const_sum_product! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
float_const_sum_product! { f32 f64 }
