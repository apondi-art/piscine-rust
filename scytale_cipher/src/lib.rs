pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    if i <= 1 {
        return message;
    }
    
    let mut chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + i - 1) / i; // Ceiling division
    let total = rows * i;
    chars.resize(total, ' ');
    
    let mut result = String::with_capacity(total);
    
    for col in 0..i {
        for row in 0..rows {
            result.push(chars[row * i + col]);
        }
    }
    
    // Trim only the padding spaces we added, not original characters
    let mut trimmed = String::with_capacity(len);
    let mut count = 0;
    for c in result.chars() {
        if count < len {
            trimmed.push(c);
            count += 1;
        } else {
            break;
        }
    }
    trimmed
}