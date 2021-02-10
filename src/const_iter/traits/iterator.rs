use super::super::adapters::{Map, Zip};
use super::accum::Sum;
use super::collect::{FromIterator, IntoIterator};

pub trait Iterator<const N: usize> {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    #[inline]
    fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter, N>
    where
        Self: Sized,
        U: IntoIterator<N>,
    {
        Zip::new(self, other.into_const_iter())
    }

    #[inline]
    fn map<B, F>(self, f: F) -> Map<Self, F, N>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }

    #[inline]
    fn collect<B: FromIterator<Self::Item, N>>(self) -> B
    where
        Self: Sized,
    {
        FromIterator::from_const_iter(self)
    }

    #[inline]
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }

    #[inline]
    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: Sum<N, Self::Item>,
    {
        unimplemented!()
    }
}
