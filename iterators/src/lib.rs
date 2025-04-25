#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
    fn next(&mut self) -> Option<Self::Item> {
        // If we've reached 1, we stop
        if self.v == 1 {
            return None;
        }
        
        // Calculate the next value in the sequence
        let next_value = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v * 3 + 1
        };
        
        // Store the current value to return
        let current = self.v;
        
        // Update the internal state for the next iteration
        self.v = next_value;
        
        // Return the current value as a Collatz struct
        Some(Collatz { v: current })
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
    // First value is already counted by the iterator
    1 + Collatz::new(n).count()
}