use crate::const_iter::Iterator;

pub struct Map<I, F, const N: usize> {
    iter: I,
    f: F,
}

impl<I, F, const N: usize> Map<I, F, N> {
    pub fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I, F, const N: usize> Iterator<N> for Map<I, F, N>
where
    I: Iterator<N>,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
}
