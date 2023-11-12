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
