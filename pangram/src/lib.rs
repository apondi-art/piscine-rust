pub fn is_pangram(s: &str) -> bool {
    let mut letters =[false;26];
    for c in s.chars(){
        if c.is_ascii_alphabetic(){
            let lower_c = c.to_ascii_lowercase();
            let index = (lower_c as u8 - b'a') as usize;
            letters[index] = true
        }
    }
    letters.iter().all(|&x| x)


}