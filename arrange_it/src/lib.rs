pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    words.sort_by_cached_key(|word| {
        word.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(0)
    });
    
    words.iter().map(|word| {
        word.chars()
            .filter(|c| !c.is_ascii_digit())
            .collect::<String>()
    }).collect::<Vec<String>>()
    .join(" ")
}
