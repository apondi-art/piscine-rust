pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    if i <= 1 {
        return message;
    }
    
    let mut chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    
    // Pad with spaces if needed to make full grid
    let padding = (i - (len % i)) % i;
    chars.extend(std::iter::repeat(' ').take(padding));
    
    let rows = chars.len() / i;
    let mut result = String::with_capacity(chars.len());
    
    for col in 0..i {
        for row in 0..rows {
            let index = row * i + col;
            result.push(chars[index]);
        }
    }
    
    result
}