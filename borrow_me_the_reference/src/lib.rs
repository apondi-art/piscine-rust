pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            '-' => {
                // Backspace: remove last character from result
                result.pop();
            },
            '+' => {
                // Delete: skip next character
                i += 1;  // Skip the next character
            },
            c => {
                // Normal character: add to result
                result.push(c);
            }
        }
        i += 1;
    }
    
    *s = result;  // Replace the original string
}


pub fn do_operations(v: &mut Vec<String>) {
    for equation in v.iter_mut() {
        if let Some(plus_pos) = equation.find('+') {
            // Handle addition
            let a = equation[..plus_pos].parse::<i32>().unwrap();
            let b = equation[plus_pos+1..].parse::<i32>().unwrap();
            *equation = (a + b).to_string();
        } else if let Some(minus_pos) = equation.find('-') {
            // Handle subtraction (but not negative numbers)
            if minus_pos > 0 {  // Ensure '-' isn't the first character
                let a = equation[..minus_pos].parse::<i32>().unwrap();
                let b = equation[minus_pos+1..].parse::<i32>().unwrap();
                *equation = (a - b).to_string();
            }
        }
    }
}