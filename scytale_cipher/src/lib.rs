pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    if i <= 1 {
        return message;
    }
    
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + i - 1) / i; // Ceiling division
    
    let mut result = String::with_capacity(len);
    let mut written = 0;
    
    for col in 0..i {
        for row in 0..rows {
            let index = row * i + col;
            if index < len {
                result.push(chars[index]);
                written += 1;
            }
            if written == len {
                return result;
            }
        }
    }
    
    result
}