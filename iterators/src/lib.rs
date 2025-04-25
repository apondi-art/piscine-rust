#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None;
        }
        
        let next_v = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v * 3 + 1
        };
        
        let current = Collatz { v: self.v };
        self.v = next_v;
        
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
    let mut count = 0;
    let mut c = Collatz::new(n);
    while let Some(_) = c.next() {
        count += 1;
    }
    count
}