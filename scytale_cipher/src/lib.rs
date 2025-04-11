pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    if i <= 1 {
        return message;
    }
    
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + i - 1) / i; // Ceiling division
    
    let mut result = String::with_capacity(len);
    let mut chars_read = 0;
    
    for col in 0..i {
        for row in 0..rows {
            let index = row * i + col;
            if index < len {
                result.push(chars[index]);
                chars_read += 1;
            }
            // Stop when we've read all original characters
            if chars_read >= len {
                break;
            }
        }
        if chars_read >= len {
            break;
        }
    }
    
    result
}