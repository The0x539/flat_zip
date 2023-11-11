#[derive(Debug, Clone)]
struct Group<K, G> {
    key: K,
    values: G,
}

impl<K, G, V> Group<K, G>
where
    G: Iterator<Item = V>,
{
    fn new(key: K, values: impl IntoIterator<IntoIter = G>) -> Self {
        Self {
            key,
            values: values.into_iter(),
        }
    }

    fn last(self) -> Option<(K, V)> {
        let value = self.values.last()?;
        Some((self.key, value))
    }
}

impl<K, G, V> Group<K, G>
where
    K: Clone,
    G: Iterator<Item = V>,
{
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, (K, V)) -> B,
    {
        self.values.fold(init, |acc, value| {
            let pair = (self.key.clone(), value);
            f(acc, pair)
        })
    }
}

#[derive(Debug, Clone)]
pub struct FlatZip<I, K, G: IntoIterator> {
    current_group: Option<Group<K, G::IntoIter>>,
    groups: I,
}

impl<I, K, G, V> FlatZip<I, K, G>
where
    K: Clone,
    I: Iterator<Item = (K, G)>,
    G: IntoIterator<Item = V>,
{
    fn next_group(&mut self) -> Option<Group<K, G::IntoIter>> {
        if let Some(group) = self.current_group.take() {
            return Some(group);
        }

        let (key, values) = self.groups.next()?;
        Some(Group::new(key, values))
    }

    fn fold_groups<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Group<K, G::IntoIter>) -> B,
    {
        let mut acc = init;

        if let Some(group) = self.current_group {
            acc = f(acc, group);
        }

        self.groups.fold(acc, |a, (key, values)| {
            let group = Group::new(key, values);
            f(a, group)
        })
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
            let Some(mut group) = self.next_group() else {
                // there are no more groups, so iteration is over
                return None;
            };

            let Some(value) = group.values.next() else {
                // this group was empty, but the next one might not be
                continue;
            };

            let key = group.key.clone();
            // save the rest of the group for later
            self.current_group = Some(group);
            return Some((key, value));
        }
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, (K, V)) -> B,
    {
        self.fold_groups(init, |acc, group| group.fold(acc, &mut f))
    }

    fn count(self) -> usize {
        self.fold_groups(0, |n, group| n + group.values.count())
    }

    fn last(self) -> Option<Self::Item> {
        self.fold_groups(None, |acc, group| group.last().or(acc))
    }
}

pub trait FlatZipExt: Iterator<Item = (Self::Key, Self::Group)> + Sized {
    type Key: Clone;
    type Group: IntoIterator;

    fn flat_zip(self) -> FlatZip<Self, Self::Key, Self::Group>;
}

impl<I, K, G> FlatZipExt for I
where
    I: Iterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator,
{
    type Key = K;
    type Group = G;

    fn flat_zip(self) -> FlatZip<Self, Self::Key, Self::Group> {
        FlatZip {
            current_group: None,
            groups: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FlatZipExt;
    use std::collections::BTreeMap;

    #[test]
    fn test_enumerated() {
        let jagged_vec = vec![
            vec!["zero", "", "0"],
            vec![],
            vec!["II", "two"],
            vec!["three", "three", "three"],
            vec![],
            vec!["55555"],
            vec!["6"],
            vec!["seven"; 2],
            vec![],
            vec![],
        ];

        let mut iter = jagged_vec.into_iter().enumerate().flat_zip();

        assert_eq!(iter.next(), Some((0, "zero")));
        assert_eq!(iter.next(), Some((0, "")));
        assert_eq!(iter.next(), Some((0, "0")));

        assert_eq!(iter.next(), Some((2, "II")));
        assert_eq!(iter.next(), Some((2, "two")));

        assert_eq!(iter.next(), Some((3, "three")));
        assert_eq!(iter.next(), Some((3, "three")));
        assert_eq!(iter.next(), Some((3, "three")));

        assert_eq!(iter.next(), Some((5, "55555")));

        assert_eq!(iter.next(), Some((6, "6")));

        assert_eq!(iter.next(), Some((7, "seven")));
        assert_eq!(iter.next(), Some((7, "seven")));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_multimap() {
        let mut map = BTreeMap::new();
        map.insert(3, vec!["three", "3", "III"]);
        map.insert(7, vec!["7", "seven", "VII"]);
        map.insert(9, vec![]);
        map.insert(8, vec!["eight", "8"]);

        let mut iter = map.into_iter().flat_zip();

        assert_eq!(iter.next(), Some((3, "three")));
        assert_eq!(iter.next(), Some((3, "3")));
        assert_eq!(iter.next(), Some((3, "III")));

        assert_eq!(iter.next(), Some((7, "7")));
        assert_eq!(iter.next(), Some((7, "seven")));
        assert_eq!(iter.next(), Some((7, "VII")));

        assert_eq!(iter.next(), Some((8, "eight")));
        assert_eq!(iter.next(), Some((8, "8")));

        assert_eq!(iter.next(), None);
    }
}
