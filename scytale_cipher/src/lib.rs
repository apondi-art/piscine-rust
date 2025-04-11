pub fn scytale_cipher(message: String, size: u32) -> String {
    let message_chars: Vec<char> = message.chars().collect();
    let message_len = message_chars.len();
    let size = size as usize;
    
    // Calculate the number of rows needed (ceiling division)
    let rows = (message_len + size - 1) / size;
    
    let mut result = String::with_capacity(message_len);
    
    // Read the message by going down each column
    for col in 0..size {
        for row in 0..rows {
            let index = row * size + col;
            
            if index < message_len {
                result.push(message_chars[index]);
            }
        }
    }
    
    result
}
