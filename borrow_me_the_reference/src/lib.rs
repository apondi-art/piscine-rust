pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    
    for c in s.chars() {
        match c {
            '-' => {
                // Backspace: remove last character if exists
                result.pop();
            },
            '+' => {
                // Delete: mark next character for deletion
                result.push('\0'); // Special marker
            },
            _ => {
                // Handle pending delete operation
                if let Some('\0') = result.last() {
                    result.pop(); // Remove the marker
                    // Don't push this character (it's being deleted)
                } else {
                    result.push(c);
                }
            }
        }
    }
    
    // Remove any remaining delete markers (from trailing '+')
    result.retain(|&c| c != '\0');
    *s = result.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for equation in v.iter_mut() {
        if let Some(plus_pos) = equation.find('+') {
            if let (Ok(a), Ok(b)) = (
                equation[..plus_pos].parse::<i32>(),
                equation[plus_pos+1..].parse::<i32>(),
            ) {
                *equation = (a + b).to_string();
            }
        } else if let Some(minus_pos) = equation.find('-') {
            if minus_pos > 0 {
                if let (Ok(a), Ok(b)) = (
                    equation[..minus_pos].parse::<i32>(),
                    equation[minus_pos+1..].parse::<i32>(),
                ) {
                    *equation = (a - b).to_string();
                }
            }
        }
    }
}