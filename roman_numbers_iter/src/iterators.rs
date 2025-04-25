use crate::{RomanDigit, RomanNumber};

impl<'a> IntoIterator for &'a RomanNumber {
    type Item = &'a RomanDigit;
    type IntoIter = std::slice::Iter<'a, RomanDigit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut RomanNumber {
    type Item = &'a mut RomanDigit;
    type IntoIter = std::slice::IterMut<'a, RomanDigit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for RomanNumber {
    type Item = RomanDigit;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}