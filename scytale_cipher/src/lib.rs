pub fn scytale_cipher(message: String, size: u32) -> String {
    if size == 1 || size as usize >= message.len() {
        return message;
    }

    let message_chars: Vec<char> = message.chars().collect();
    let message_len = message_chars.len();
    let rows = size as usize;
    
    // Calculate number of columns (width) based on ceiling of length / rows
    let width = (message_len + rows - 1) / rows;
    
    let mut result = String::with_capacity(message_len);
    
    // Read the message by going through each row and then each column
    for row in 0..rows {
        for col in 0..width {
            let index = col * rows + row;
            if index < message_len {
                result.push(message_chars[index]);
            } else {
                // This handles padding with spaces when needed
                result.push(' ');
            }
        }
    }
    
    // Trim trailing spaces to match friend's implementation
    result.trim_end().to_string()
}