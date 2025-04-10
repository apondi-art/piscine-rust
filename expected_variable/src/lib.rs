use case::{Case, CaseExt}; // Import the case crate

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if compared string is in camelCase or snake_case using the case crate
    let is_camel = compared.is_camel();
    let is_snake = compared.is_snake();
    
    // If the compared string is not in camel case or snake case, return None
    if !is_camel && !is_snake {
        return None;
    }
    
    // Case-insensitive comparison
    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();
    
    // Use the edit_distance function defined in the same file
    let distance = edit_distance(&compared_lower, &expected_lower);
    let expected_len = expected_lower.len();
    
    // Avoid division by zero
    if expected_len == 0 {
        return None;
    }
    
    // Calculate similarity percentage
    let similarity = ((expected_len as f64 - distance as f64) / expected_len as f64) * 100.0;
    let similarity_rounded = similarity.round() as u32;
    
    // If similarity is greater than 50%, return the percentage, otherwise None
    (similarity_rounded > 50).then(|| format!("{}%", similarity_rounded))
}

// The edit_distance function definition should already be in your lib.rs
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