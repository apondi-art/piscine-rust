// Make sum work with any slice of i32
pub fn sum(a: &[i32]) -> i32 {
    let mut total = 0;
    for num in a {
        total += num;
    }
    total
}

// Keep thirtytwo_tens as specified
pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}