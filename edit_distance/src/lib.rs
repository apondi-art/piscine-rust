pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let source_len = source_chars.len();
    let target_len = target_chars.len();

    // Initialize the previous row (edit distances for an empty source string)
    let mut prev_row: Vec<usize> = (0..=target_len).collect();

    for i in 1..=source_len {
        // Initialize the current row with the deletion cost
        let mut curr_row = vec![i; target_len + 1];

        for j in 1..=target_len {
            let cost = if source_chars[i - 1] == target_chars[j - 1] {
                prev_row[j - 1]  // No operation needed
            } else {
                1 + prev_row[j - 1].min(prev_row[j].min(curr_row[j - 1])) // Substitution, deletion, or insertion
            };
            curr_row[j] = cost;
        }

        prev_row = curr_row;
    }

    prev_row[target_len]
}