// iterators.rs
use crate::{RomanDigit, RomanNumber};

// Implement IntoIterator for owned values
impl IntoIterator for RomanNumber {
    type Item = RomanDigit;
    type IntoIter = std::vec::IntoIter<RomanDigit>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Implement IntoIterator for references
impl<'a> IntoIterator for &'a RomanNumber {
    type Item = &'a RomanDigit;
    type IntoIter = std::slice::Iter<'a, RomanDigit>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// Implement IntoIterator for mutable references
impl<'a> IntoIterator for &'a mut RomanNumber {
    type Item = &'a mut RomanDigit;
    type IntoIter = std::slice::IterMut<'a, RomanDigit>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

// Implement Iterator trait to increment Roman numbers
impl Iterator for RomanNumber {
    type Item = RomanNumber;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current_value = self.to_u32();
        let next_value = current_value + 1;
        *self = RomanNumber::from(next_value);
        Some(self.clone())
    }
}

impl RomanNumber {
    // Helper method to convert to u32
    pub fn to_u32(&self) -> u32 {
        self.0.iter().map(|d| match d {
            RomanDigit::Nulla => 0,
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
        }).sum()
    }
}