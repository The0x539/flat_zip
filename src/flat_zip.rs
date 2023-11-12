use crate::{group::Group, groups::Groups};
use std::iter::Fuse;

pub struct FlatZip<I, K, G>
where
    I: Iterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
    // see FlattenCompat from std
    groups: Fuse<Groups<I>>,
    front: Option<Group<K, G::IntoIter>>,
    back: Option<Group<K, G::IntoIter>>,
}

impl<I, K, G> FlatZip<I, K, G>
where
    I: Iterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
    pub fn new<T: IntoIterator<IntoIter = I>>(iter: T) -> Self {
        Self {
            groups: Groups::new(iter).fuse(),
            front: None,
            back: None,
        }
    }

    fn fold_groups<B, F>(self, mut acc: B, mut f: F) -> B
    where
        F: FnMut(B, Group<K, G::IntoIter>) -> B,
    {
        if let Some(group) = self.front {
            acc = f(acc, group);
        }

        acc = self.groups.fold(acc, &mut f);

        if let Some(group) = self.back {
            acc = f(acc, group);
        }

        acc
    }

    fn rfold_groups<B, F>(self, mut acc: B, mut f: F) -> B
    where
        F: FnMut(B, Group<K, G::IntoIter>) -> B,
        I: DoubleEndedIterator,
        G::IntoIter: DoubleEndedIterator,
    {
        if let Some(group) = self.back {
            acc = f(acc, group);
        }

        acc = self.groups.rfold(acc, &mut f);

        if let Some(group) = self.front {
            acc = f(acc, group);
        }

        acc
    }
}

impl<I, K, G, V> Iterator for FlatZip<I, K, G>
where
    I: Iterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator<Item = V>,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(group) = &mut self.front {
                if let Some(item) = group.next() {
                    break Some(item);
                }
                self.front = None;
            }

            if let Some(group) = self.groups.next() {
                self.front = Some(group);
                continue;
            }

            if let Some(group) = &mut self.back {
                if let Some(item) = group.next() {
                    break Some(item);
                }
                self.back = None;
            }

            break None;
        }
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.fold_groups(init, |acc, group| group.fold(acc, &mut f))
    }

    fn count(self) -> usize {
        self.fold_groups(0, |n, group| n + group.count())
    }

    fn last(self) -> Option<Self::Item> {
        self.fold_groups(None, |acc, group| group.last().or(acc))
    }
}

impl<I, K, G, V> DoubleEndedIterator for FlatZip<I, K, G>
where
    I: DoubleEndedIterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator<Item = V>,
    G::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(group) = &mut self.back {
                if let Some(item) = group.next_back() {
                    break Some(item);
                }
                self.back = None;
            }

            if let Some(group) = self.groups.next_back() {
                self.back = Some(group);
                continue;
            }

            if let Some(group) = &mut self.front {
                if let Some(item) = group.next_back() {
                    break Some(item);
                }
                self.front = None;
            }

            break None;
        }
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.rfold_groups(init, |acc, group| group.rfold(acc, &mut f))
    }
}
