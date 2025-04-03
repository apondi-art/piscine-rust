pub fn first_subword(s: String) -> String {
    s.chars()
        .take_while(|&c| c.is_lowercase() && c != '_')
        .collect()
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
