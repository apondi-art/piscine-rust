#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl Numbers {
    pub fn new(numbers: &[u32]) -> Self {
        Numbers { numbers}
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted: Vec<u32> = self.numbers.to_vec();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        sorted.truncate(3);
        sorted
    }
}
