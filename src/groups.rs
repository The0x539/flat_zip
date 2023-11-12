use std::iter::FusedIterator;

use crate::group::Group;

#[derive(Debug, Clone)]
pub struct Groups<I> {
    iter: I,
}

impl<I, K, G> Groups<I>
where
    I: Iterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
    pub fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<IntoIter = I>,
    {
        let iter = iter.into_iter();
        Self { iter }
    }
}

impl<I, K, G> Iterator for Groups<I>
where
    I: Iterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
    type Item = Group<K, G::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Group::from_pair)
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.map(Group::from_pair).fold(init, f)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(Group::from_pair)
    }

    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(Group::from_pair)
    }
}

impl<I, K, G> DoubleEndedIterator for Groups<I>
where
    I: DoubleEndedIterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Group::from_pair)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Group::from_pair)
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.map(Group::from_pair).rfold(init, f)
    }
}

impl<I, K, G> ExactSizeIterator for Groups<I>
where
    I: ExactSizeIterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I, K, G> FusedIterator for Groups<I>
where
    I: FusedIterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
}
