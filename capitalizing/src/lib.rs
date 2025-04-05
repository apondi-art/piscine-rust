pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let capitalized = first.to_uppercase().collect::<String>();
            capitalized + chars.as_str()
        }
    }
}

pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let capitalized = first.to_uppercase().collect::<String>();
                    let rest = chars.collect::<String>().to_lowercase();
                    capitalized + &rest
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
}