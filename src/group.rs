use std::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct Group<K, I> {
    key: K,
    values: I,
}

impl<K: Clone, I> Group<K, I> {
    pub fn new<G>(key: K, values: G) -> Self
    where
        G: IntoIterator<IntoIter = I>,
    {
        let values = values.into_iter();
        Self { key, values }
    }

    pub fn from_pair<G>((key, values): (K, G)) -> Self
    where
        G: IntoIterator<IntoIter = I>,
    {
        Self::new(key, values)
    }
}

impl<K, I, V> Iterator for Group<K, I>
where
    K: Clone,
    I: Iterator<Item = V>,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next().map(|v| (self.key.clone(), v))
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, (K, V)) -> B,
    {
        self.values.fold(init, |acc, value| {
            let pair = (self.key.clone(), value);
            f(acc, pair)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<(K, V)> {
        self.values.nth(n).map(|v| (self.key.clone(), v))
    }

    fn last(self) -> Option<(K, V)> {
        self.values.last().map(|v| (self.key, v))
    }

    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        let mut pair = (self.key.clone(), self.values.next()?);
        loop {
            if predicate(&pair) {
                return Some(pair);
            }
            // avoid cloning the key more than once
            pair.1 = self.values.next()?;
        }
    }
}

impl<K, I, V> DoubleEndedIterator for Group<K, I>
where
    K: Clone,
    I: DoubleEndedIterator<Item = V>,
{
    fn next_back(&mut self) -> Option<(K, V)> {
        self.values.next_back().map(|v| (self.key.clone(), v))
    }

    fn nth_back(&mut self, n: usize) -> Option<(K, V)> {
        self.values.nth_back(n).map(|v| (self.key.clone(), v))
    }

    fn rfold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, (K, V)) -> B,
    {
        self.values.rfold(init, |acc, value| {
            let pair = (self.key.clone(), value);
            f(acc, pair)
        })
    }

    fn rfind<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        let mut pair = self.next()?;
        loop {
            if predicate(&pair) {
                return Some(pair);
            }
            // avoid cloning the key more than once
            pair.1 = self.values.next_back()?;
        }
    }
}

impl<K, I> ExactSizeIterator for Group<K, I>
where
    K: Clone,
    I: ExactSizeIterator,
{
    fn len(&self) -> usize {
        self.values.len()
    }
}

impl<K, I> FusedIterator for Group<K, I>
where
    K: Clone,
    I: FusedIterator,
{
}
