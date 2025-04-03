pub fn first_subword(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_uppercase() => {
            // For uppercase-starting words, take until '_'
            let rest: String = chars.take_while(|&c| c != '_').collect();
            format!("{}{}", c, rest)
        }
        Some(c) => {
            // For lowercase-starting words, take until uppercase or '_'
            let rest: String = chars.take_while(|&c| c.is_lowercase() && c != '_').collect();
            format!("{}{}", c, rest)
        }
        None => String::new(), // empty string case
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
