#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
    has_returned_self: bool,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
    fn next(&mut self) -> Option<Self::Item> {
        // If we've already returned ourselves once, calculate the next value
        if self.has_returned_self {
            // If we've reached 1, the sequence ends
            if self.v == 1 {
                return None;
            }
            
            // Calculate the next value
            self.v = if self.v % 2 == 0 {
                self.v / 2
            } else {
                self.v * 3 + 1
            };
        } else {
            // Mark that we've returned our initial value
            self.has_returned_self = true;
        }
        
        // Return the current value
        Some(Collatz { v: self.v, has_returned_self: true })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { 
            v: n,
            has_returned_self: false
        }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 || n == 1 {
        return 0;
    }
    Collatz::new(n).count()
}