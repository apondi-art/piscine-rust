pub fn rotate(input: &str, key: i8) -> String {
    let mut str_final = String::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' } as i32;
            let current_pos = c as i32 - base;
            let rotated_pos = (current_pos + key as i32).rem_euclid(26);
            let rotated_char = (base + rotated_pos) as u8 as char;
            str_final.push(rotated_char);
        } else {
            str_final.push(c);
        }
    }
    str_final
}