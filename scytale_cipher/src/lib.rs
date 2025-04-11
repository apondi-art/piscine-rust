pub fn scytale_cipher(message: String, size: u32) -> String {
    let message_len = message.len();
    
    // If the size is greater than the message length or is zero, return the original message
    if size == 0 || size as usize >= message_len {
        return message;
    }
    
    // Calculate the number of rows we need
    // This is effectively the number of times the strip is wrapped around the cylinder
    let rows = (message_len as f32 / size as f32).ceil() as usize;
    
    let mut result = String::with_capacity(message_len);
    
    // Read the message by going down each column
    for col in 0..size as usize {
        for row in 0..rows {
            let index = row * size as usize + col;
            
            // Only add the character if the index is within the message bounds
            if index < message_len {
                result.push(message.chars().nth(index).unwrap());
            }
        }
    }
    
    result
}

