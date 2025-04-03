pub fn first_subword(s: String) -> String {
    let mut result = String::new();
    let mut first_char = true;
    
    for c in s.chars() {
        if c == '_' {
            break;
        }
        
        if !first_char && c.is_uppercase() {
            break;
        }
        
        result.push(c);
        first_char = false;
    }
    
    result
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
