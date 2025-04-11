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
    
    result.truncate(len);
    result
}