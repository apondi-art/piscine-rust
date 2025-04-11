pub fn num_to_ordinal(x: u32) -> String {
    // Handle special cases for numbers ending with 11, 12, or 13
    let suffix = match x % 100 {
        11 | 12 | 13 => "th",
        _ => match x % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", x, suffix)
}
