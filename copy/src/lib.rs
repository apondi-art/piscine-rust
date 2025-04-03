pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let original = c;
    let exp = (c as f64).exp();
    let ln = if c == 0 {
        f64::NEG_INFINITY
    } else {
        (c.abs() as f64).ln()
    };
    (original, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let original = a.clone();
    let parts: Vec<&str> = a.split_whitespace().collect();
    let mut exp_parts = Vec::new();
    for part in parts {
        let num = part.parse::<i32>().unwrap();
        let exp = (num as f64).exp();
        exp_parts.push(exp.to_string());
    }
    let exp_str = exp_parts.join(" ");
    (original, exp_str)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let original = b.clone();
    let mut ln_values = Vec::new();
    for &num in &b {
        let ln = if num == 0 {
            f64::NEG_INFINITY
        } else {
            (num.abs() as f64).ln()
        };
        ln_values.push(ln);
    }
    (original, ln_values)
}