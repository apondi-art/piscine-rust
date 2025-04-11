pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    if i <= 1 {
        return message;
    }
    
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    
    // Calculate number of full rows we need
    let rows = (len + i - 1) / i;
    
    // Create an empty 2D grid filled with spaces
    let mut grid = vec![vec![' '; i]; rows];
    
    // Fill the grid row by row with the message characters
    let mut char_idx = 0;
    for row in 0..rows {
        for col in 0..i {
            if char_idx < len {
                grid[row][col] = chars[char_idx];
                char_idx += 1;
            }
        }
    }
    
    // Read the grid column by column to create the cipher
    let mut result = String::new();
    for col in 0..i {
        for row in 0..rows {
            // Only add characters that are within the original message length
            let original_idx = row * i + col;
            if original_idx < len {
                result.push(grid[row][col]);
            }
        }
    }
    
    result
}