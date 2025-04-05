use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values().filter(|&&v| v > 0).max().copied().unwrap_or(0)
}