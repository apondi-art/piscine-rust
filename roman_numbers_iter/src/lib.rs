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
    fn from(n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut quotient = n;
        let mut place = 0;
        let mut reverse_roman = Vec::new();

        while quotient != 0 {
            let remainder = quotient % 10;
            quotient /= 10;
            place += 1;
            
            match remainder {
                4 => {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(place)/2));
                    reverse_roman.push(RomanDigit::from(10_u32.pow(place - 1)));
                },
                9 => {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(place)));
                    reverse_roman.push(RomanDigit::from(10_u32.pow(place - 1)));
                },
                5..=8 => {
                    for _ in 0..(remainder - 5) {
                        reverse_roman.push(RomanDigit::from(10_u32.pow(place - 1)));
                    }
                    reverse_roman.push(RomanDigit::from(10_u32.pow(place)/2));
                },
                1..=3 => {
                    for _ in 0..remainder {
                        reverse_roman.push(RomanDigit::from(10_u32.pow(place - 1)));
                    }
                },
                _ => {},
            }
        }

        reverse_roman.reverse();
        RomanNumber(reverse_roman)
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
        let next_value = current_value + 1;
        *self = RomanNumber::from(next_value);
        Some(self.clone())
    }
}