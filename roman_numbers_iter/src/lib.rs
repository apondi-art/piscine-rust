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
            1..=4 => RomanDigit::I,
            5..=9 => RomanDigit::V,
            10..=49 => RomanDigit::X,
            50..=99 => RomanDigit::L,
            100..=499 => RomanDigit::C,
            500..=999 => RomanDigit::D,
            1000..=5000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut result = Vec::new();
        let values = [
            (1000, RomanDigit::M),
            (900, RomanDigit::C),
            (500, RomanDigit::D),
            (400, RomanDigit::C),
            (100, RomanDigit::C),
            (90, RomanDigit::X),
            (50, RomanDigit::L),
            (40, RomanDigit::X),
            (10, RomanDigit::X),
            (9, RomanDigit::I),
            (5, RomanDigit::V),
            (4, RomanDigit::I),
            (1, RomanDigit::I),
        ];

        for (value, digit) in values {
            while n >= value {
                n -= value;
                result.push(digit);
            }
        }

        RomanNumber(result)
    }
}

impl RomanNumber {
    pub fn to_u32(&self) -> u32 {
        self.0.iter().fold(0, |acc, digit| {
            acc + match digit {
                RomanDigit::Nulla => 0,
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
            }
        })
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current_value = self.to_u32();
        let next_num = RomanNumber::from(current_value + 1);
        *self = next_num.clone();
        Some(next_num)
    }
}