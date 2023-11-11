#[derive(Debug, Clone)]
struct Group<K, G> {
    key: K,
    values: G,
}

#[derive(Debug, Clone)]
pub struct FlatZip<I, K, T: IntoIterator> {
    current_group: Option<Group<K, T::IntoIter>>,
    groups: I,
}

impl<I, K, G, V> Iterator for FlatZip<I, K, G>
where
    I: Iterator<Item = (K, G)>,
    K: Clone,
    G: IntoIterator<Item = V>,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(group) = &mut self.current_group {
            if let Some(value) = group.values.next() {
                return Some((group.key.clone(), value));
            }
        }

        // self.current_group is either absent or empty
        // go through self.groups until something can produce a value
        loop {
            let Some((key, values)) = self.groups.next() else {
                // there are no more groups, so iteration is over
                return None;
            };

            let mut values = values.into_iter();
            let Some(value) = values.next() else {
                // this group was empty, but the next one might not be
                continue;
            };

            let k = key.clone();
            // save the rest of the group for later
            self.current_group = Some(Group { key, values });

            return Some((k, value));
        }
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
