
pub fn spell(n: u64) -> String {
    if n == 0{
        return "zero".to_string();
    }

    convert(n)

}

fn convert(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        21..=99 => {
            let tens = (n / 10) * 10;
            let units = n % 10;
            format!("{}-{}", convert(tens), convert(units))
        }
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} hundred", convert(hundreds))
            } else {
                format!("{} hundred {}", convert(hundreds), convert(remainder))
            }
        }
        1000..=999_999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            if remainder == 0 {
                format!("{} thousand", convert(thousands))
            } else {
                format!("{} thousand {}", convert(thousands), convert(remainder))
            }
        }
        1_000_000 => "one million".to_string(),
        _ => "".to_string(),
    }
}