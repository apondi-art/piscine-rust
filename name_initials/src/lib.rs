pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter()
        .map(|name| {
            let mut result = String::new();
            for (i, word) in name.split_whitespace().enumerate() {
                if let Some(c) = word.chars().next() {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push(c.to_ascii_uppercase());
                    result.push('.');
                }
            }
            result
        })
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
