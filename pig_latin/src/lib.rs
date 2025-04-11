pub fn pig_latin(text: &str) -> String {

    text.split_whitespace()
    .map(|word|{
        let mut chars :Vec<char> = word.chars().collect();
        //check for consonant the followed by qu
        if chars.len() > 3{
            let first = chars[0];
           let  second = chars[1];
           let third = chars[2];
           if !is_vowel(first) && second.to_ascii_lowercase() == 'q' && third.to_ascii_lowercase() == 'u' {
            let moved : Vec<_> = chars.drain(0..3).collect();
            chars.extend(moved);
            chars.extend(vec!['a','y']);
            return chars.into_iter().collect();
           }

        }
        //checks with vowel
        if !chars.is_empty() && is_vowel(chars[0]){
            chars.extend(vec!['a', 'y']);
            return chars.into_iter().collect()
        }
        //handle consonantcases
        if let Some(pos) = find_first_vowel(&chars){
            let moved : Vec<_> = chars.drain(0..pos).collect();
            chars.extend(moved);
            chars.extend(vec!['a','y']);

        }else{
            chars.extend(vec!['a','y'])
        }
        chars.into_iter().collect()


    })

  .collect::<Vec<String>>()
        .join(" ")

}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn find_first_vowel(chars: &[char]) -> Option<usize> {
    chars.iter().position(|&c| is_vowel(c))
}