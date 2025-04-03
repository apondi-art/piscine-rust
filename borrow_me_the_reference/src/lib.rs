pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut to_skip = 0;
    
    for c in s.chars() {
        if to_skip > 0 {
            to_skip -= 1;
            continue;
        }
        
        match c {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            },
            '+' => {
                to_skip = 1;
            },
            _ => {
                result.push(c);
            }
        }
    }
    
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