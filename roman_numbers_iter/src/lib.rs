
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
        
        // Handle thousands
        while n >= 1000 {
            result.push(RomanDigit::M);
            n -= 1000;
        }
        
        // Handle hundreds
        if n >= 900 {
            result.push(RomanDigit::C);
            result.push(RomanDigit::M);
            n -= 900;
        } else if n >= 500 {
            result.push(RomanDigit::D);
            n -= 500;
            while n >= 100 {
                result.push(RomanDigit::C);
                n -= 100;
            }
        } else if n >= 400 {
            result.push(RomanDigit::C);
            result.push(RomanDigit::D);
            n -= 400;
        } else {
            while n >= 100 {
                result.push(RomanDigit::C);
                n -= 100;
            }
        }
        
        // Handle tens
        if n >= 90 {
            result.push(RomanDigit::X);
            result.push(RomanDigit::C);
            n -= 90;
        } else if n >= 50 {
            result.push(RomanDigit::L);
            n -= 50;
            while n >= 10 {
                result.push(RomanDigit::X);
                n -= 10;
            }
        } else if n >= 40 {
            result.push(RomanDigit::X);
            result.push(RomanDigit::L);
            n -= 40;
        } else {
            while n >= 10 {
                result.push(RomanDigit::X);
                n -= 10;
            }
        }
        
        // Handle ones
        if n >= 9 {
            result.push(RomanDigit::I);
            result.push(RomanDigit::X);
            n -= 9;
        } else if n >= 5 {
            result.push(RomanDigit::V);
            n -= 5;
            while n >= 1 {
                result.push(RomanDigit::I);
                n -= 1;
            }
        } else if n >= 4 {
            result.push(RomanDigit::I);
            result.push(RomanDigit::V);
            n -= 4;
        } else {
            while n >= 1 {
                result.push(RomanDigit::I);
                n -= 1;
            }
        }
        
        RomanNumber(result)
    }
}

impl RomanNumber {
    pub fn to_u32(&self) -> u32 {
        let mut result = 0;
        let mut prev_value = 0;
        
        for digit in self.0.iter().rev() {
            let current_value = match digit {
                RomanDigit::Nulla => 0,
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
            };
            
            if current_value >= prev_value {
                result += current_value;
            } else {
                result -= current_value;
            }
            
            prev_value = current_value;
        }
        
        result
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
