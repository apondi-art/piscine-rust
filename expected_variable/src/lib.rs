use case::CaseExt;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Validate camel/snake case using case crate
    let is_camel = compared.to_camel() == compared;
    let is_snake = compared.to_snake() == compared;
    
    if !is_camel && !is_snake {
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

    let similarity = ((expected_len - distance) as f64 / expected_len as f64) * 100.0;
    
    (similarity > 50.0).then(|| format!("{:.0}%", similarity))
}

// Add this edit distance implementation
fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

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