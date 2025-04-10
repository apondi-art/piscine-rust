pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if the compared string is in camel case or snake case
    let is_camel = is_camel_case(compared);
    let is_snake = is_snake_case(compared);
    
    if !is_camel && !is_snake {
        return None;
    }

    // Case-insensitive comparison by converting both strings to lowercase
    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();

    let distance = edit_distance(&compared_lower, &expected_lower);
    
    let expected_len = expected_lower.len();
    if expected_len == 0 {
        return None;
    }

    // Calculate similarity percentage
    let similarity = ((expected_len as f64 - distance as f64) / expected_len as f64) * 100.0;
    let similarity_rounded = similarity.round() as u32;

    // Return formatted percentage if above 50%
    (similarity_rounded > 50).then(|| format!("{}%", similarity_rounded))
}

// Helper function to check if a string is in camelCase
fn is_camel_case(s: &str) -> bool {
    !s.contains('_') && 
    s.chars().any(|c| c.is_uppercase()) && 
    s.chars().all(|c| c.is_alphanumeric())
}

// Helper function to check if a string is in snake_case
fn is_snake_case(s: &str) -> bool {
    if s.is_empty() || !s.contains('_') {
        return false;
    }
    
    let parts: Vec<&str> = s.split('_').collect();
    if parts.iter().any(|&p| p.is_empty()) {
        return false;
    }
    
    parts.iter().all(|p| p.chars().all(|c| c.is_alphanumeric()))
}

// Helper function to compute the Levenshtein distance between two strings
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