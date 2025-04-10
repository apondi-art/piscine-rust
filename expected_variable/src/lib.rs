pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Enhanced case validation
    let compared_is_snake = compared.contains('_');
    let compared_is_camel = !compared_is_snake && compared.chars().any(|c| c.is_uppercase());
    
    // Return None if compared doesn't follow a naming convention
    if !compared_is_camel && !compared_is_snake {
        return None;
    }
    
    // Check expected string's naming convention
    let expected_is_snake = expected.contains('_');
    let expected_is_camel = !expected_is_snake && expected.chars().any(|c| c.is_uppercase());
    
    // If they don't use the same naming convention, return None
    if (compared_is_snake && !expected_is_snake) || (compared_is_camel && !expected_is_camel) {
        return None;
    }
    
    // Case-insensitive comparison
    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();
    let distance = edit_distance(&compared_lower, &expected_lower);
    let expected_len = expected_lower.len();
    
    if expected_len == 0 {
        return None;
    }
    
    let similarity = ((expected_len as f64 - distance as f64) / expected_len as f64) * 100.0;
    let similarity_rounded = similarity.round() as u32;
    (similarity_rounded > 50).then(|| format!("{}%", similarity_rounded))
}

// Implement edit_distance in the same file
fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let (a_len, b_len) = (a_chars.len(), b_chars.len());
    let mut dp = vec![vec![0; b_len + 1]; a_len + 1];
    
    for i in 0..=a_len {
        dp[i][0] = i;
    }
    
    for j in 0..=b_len {
        dp[0][j] = j;
    }
    
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    
    dp[a_len][b_len]
}