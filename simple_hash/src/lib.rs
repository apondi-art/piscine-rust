use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut word_counts = HashMap::new();
    
    for word in words {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    
    word_counts
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}