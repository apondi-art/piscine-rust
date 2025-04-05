pub fn sum(a: [i32; 32]) -> i32 {
    let mut total = 0;
    for num in a {
        total += num;
    }
    total
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}