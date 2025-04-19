mod iterators;
use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1..=4 => I,
            5..=9 => V,
            10..=49 => X,
            50..=99 => L,
            100..=499 => C,
            500..=999 => D,
            1000..=5000 => M,
            _ => Nulla,
        }
    }
}

#[derive(Debug)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut quotient = n;
        let mut p = 0;
        let mut reverse_roman = Vec::new();

        while quotient != 0 {
            let rest = quotient % 10;
            quotient /= 10;
            p += 1;
            if rest == 9 {
                reverse_roman.push(RomanDigit::from(10_u32.pow(p)));
                reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
            } else if rest == 4 {
                reverse_roman.push(RomanDigit::from(10_u32.pow(p) / 2));
                reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
            } else if rest >= 5 {
                let repetitions = rest - 5;
                for _ in 0..repetitions {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
                }
                reverse_roman.push(RomanDigit::from(10_u32.pow(p) / 2));
            } else {
                for _ in 0..rest {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)))
                }
            }
        }

        reverse_roman.reverse();
        RomanNumber(reverse_roman)
    }
}
