use crate::const_iter::Iterator;

pub struct Zip<A, B, const N: usize> {
    a: A,
    b: B,
}

impl<A, B, const N: usize> Zip<A, B, N> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B, const N: usize> Iterator<N> for Zip<A, B, N>
where
    A: Iterator<N>,
    B: Iterator<N>,
{
    type Item = (A::Item, B::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.a.next()?;
        let b = self.b.next()?;
        Some((a, b))
    }
}
