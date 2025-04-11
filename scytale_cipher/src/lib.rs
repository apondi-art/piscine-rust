pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    if i <= 1 {
        return message;
    }
    
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + i - 1) / i; // Ceiling division
    
    // Create a 2D grid and fill it with the characters
    let mut grid = vec![vec![' '; i]; rows];
    let mut char_index = 0;
    
    for row in 0..rows {
        for col in 0..i {
            if char_index < len {
                grid[row][col] = chars[char_index];
                char_index += 1;
            }
        }
    }
    
    // Read the grid column by column
    let mut result = String::with_capacity(len);
    for col in 0..i {
        for row in 0..rows {
            if col < i && row < rows {
                result.push(grid[row][col]);
            }
        }
    }
    
    // Trim any extra spaces at the end to match exactly the length of the input
    result.truncate(len);
    
    result
}