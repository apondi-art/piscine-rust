use case::CaseExt;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if the compared string is in camel case or snake case using the case crate
    let is_camel = compared == compared.to_camel();
    let is_snake = compared == compared.to_snake();
    
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

// Helper function to compute the Levenshtein distance between two strings
fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    // Initialize a 2D vector for dynamic programming
    let mut dp = vec![vec![0; b_len + 1]; a_len + 1];

    // Base cases: transforming an empty string requires inserting all characters
    for i in 0..=a_len {
        dp[i][0] = i;
    }
    for j in 0..=b_len {
        dp[0][j] = j;
    }

    // Fill the DP table
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1) // Deletion
                .min(dp[i][j - 1] + 1)     // Insertion
                .min(dp[i - 1][j - 1] + cost); // Substitution
        }
    }

    dp[a_len][b_len]
}