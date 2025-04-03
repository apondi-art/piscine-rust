pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            '-' => {
                result.pop();
            },
            '+' => {
                i += 1;  // Skip the next character
            },
            c => {
                result.push(c);
            }
        }
        i += 1;
    }
    
    *s = result;
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