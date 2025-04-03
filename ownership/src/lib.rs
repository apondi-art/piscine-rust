pub fn first_subword(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first_char) => {
            let mut result = String::new();
            result.push(first_char);
            
            if first_char.is_lowercase() {
                // For lowercase-starting words: take until uppercase or '_'
                result.extend(chars.take_while(|&c| c.is_lowercase() && c != '_'));
            } else {
                // For uppercase-starting words: take until '_'
                result.extend(chars.take_while(|&c| c != '_'));
            }
            result
        }
        None => String::new(), // handle empty string case
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
