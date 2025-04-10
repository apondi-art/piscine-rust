pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if the compared string is in camel case or snake case
    let is_camel_case = compared.chars().any(|c| c.is_uppercase()) && !compared.contains('_');
    let is_snake_case = compared.contains('_');
    if !(is_camel_case || is_snake_case) {
        return None;
    }

    // Convert to lowercase for case-insensitive comparison
    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();

    // Compute the edit distance between the two lowercase strings
    let distance = edit_distance(&compared_lower, &expected_lower);

    // Calculate the alikeness percentage
    let expected_len = expected_lower.len();
    if expected_len == 0 {
        return None;
    }

    let similarity = ((expected_len as f64 - distance as f64) / expected_len as f64) * 100.0;
    let similarity_rounded = similarity.round() as u32;

    if similarity_rounded > 50 {
        Some(format!("{}%", similarity_rounded))
    } else {
        None
    }
}

// Levenshtein distance implementation
fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    // Initialize a 2D vector to store distances
    let mut dp = vec![vec![0; b_len + 1]; a_len + 1];

    // Base cases
    for i in 0..=a_len {
        dp[i][0] = i;
    }
    for j in 0..=b_len {
        dp[0][j] = j;
    }

    // Fill the dp table
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[a_len][b_len]
}