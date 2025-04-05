use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_counts = HashMap::new();

    // Count characters in s1
    for c in s1.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    // Decrement counts for characters in s2
    for c in s2.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }

    // Check all counts are zero
    char_counts.values().all(|&count| count == 0)
}