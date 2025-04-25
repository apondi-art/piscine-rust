#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Return the current value as a Collatz struct
        let current = Collatz { v: self.v };
        
        // If we're at 1, this is the last item we'll return
        if self.v == 1 {
            self.v = 0; // Set to 0 so next call will return None
            return Some(current);
        }
        
        // Calculate and update to the next value in the sequence
        self.v = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v * 3 + 1
        };
        
        Some(current)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 0;  // Special case: sequence doesn't continue past 1
    }
    Collatz::new(n).count() // The iterator will include all values including the terminal 1
}