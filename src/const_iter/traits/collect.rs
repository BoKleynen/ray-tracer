use crate::const_iter::Iterator;

pub trait FromIterator<A, const N: usize> {
    fn from_const_iter<I: Iterator<N, Item = A>>(iter: I) -> Self;
}

pub trait IntoIterator<const N: usize> {
    type Item;
    type IntoIter: Iterator<N, Item = Self::Item>;

    fn into_const_iter(self) -> Self::IntoIter;
}

impl<'a, I, const N: usize> IntoIterator<N> for I
where
    I: Iterator<N>,
{
    type Item = I::Item;
    type IntoIter = Self;

    fn into_const_iter(self) -> Self::IntoIter {
        self
    }
}
