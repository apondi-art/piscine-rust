use std::ops::Add;
use std::cmp::PartialOrd;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    done: bool,
}

impl<T> StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Clone,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            done: false,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current.clone();

        // Calculate the next value
        let next_value = self.current.clone() + self.step.clone();

        // Check if we've reached or passed the end
        if next_value > self.end {
            self.done = true;
        } else {
            self.current = next_value;
        }

        Some(result)
    }
}