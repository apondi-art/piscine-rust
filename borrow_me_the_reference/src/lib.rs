pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut skip_next = false;
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    
    while i < chars.len() {
        if skip_next {
            skip_next = false;
            i += 1;
            continue;
        }
        
        match chars[i] {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            },
            '+' => {
                skip_next = true;
            },
            c => {
                result.push(c);
            }
        }
        i += 1;
    }
    
    *s = result.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {  // Changed parameter type
    for equation in v.iter_mut() {
        if let Some(plus_pos) = equation.find('+') {
            let a = equation[..plus_pos].parse::<i32>().unwrap();
            let b = equation[plus_pos+1..].parse::<i32>().unwrap();
            *equation = (a + b).to_string();
        } else if let Some(minus_pos) = equation.find('-') {
            if minus_pos > 0 {
                let a = equation[..minus_pos].parse::<i32>().unwrap();
                let b = equation[minus_pos+1..].parse::<i32>().unwrap();
                *equation = (a - b).to_string();
            }
        }
    }
}