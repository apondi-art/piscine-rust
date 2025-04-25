#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;  // Change the Item type from u64 to Collatz
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None;
        }
        
        self.v = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v * 3 + 1
        };
        
        Some(Collatz { v: self.v })  // Return a new Collatz instance
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 || n == 1 {
        return 0;
    }
    Collatz::new(n).count()
}