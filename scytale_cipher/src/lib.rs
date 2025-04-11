pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    if i <= 1 {
        return message;
    }
    
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + i - 1) / i; // Ceiling division
    
    let mut result = String::with_capacity(len);
    
    for col in 0..i {
        for row in 0..rows {
            let index = row * i + col;
            if index < len {
                result.push(chars[index]);
            } else if row == rows - 1 && col < len % i && len % i != 0 {
                // Only add space at end of incomplete columns
                result.push(' ');
            }
        }
    }
    
    result
}