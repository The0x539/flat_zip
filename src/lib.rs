mod flat_zip;
mod group;
mod groups;

#[cfg(test)]
mod tests;

use flat_zip::FlatZip;

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
        FlatZip::new(self)
    }
}
