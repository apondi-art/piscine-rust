#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;  // Yield Collatz structs
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None;
        }
        
        // Calculate next value
        let next_v = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v * 3 + 1
        };
        
        // Create new Collatz for current state
        let current = Collatz { v: self.v };
        
        // Update internal state
        self.v = next_v;
        
        Some(current)  // Return current state before update
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
    Collatz::new(n).count() + 1  // +1 to include initial value
}