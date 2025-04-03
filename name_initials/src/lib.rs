pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter()
        .map(|name| {
            name.split_whitespace()
                .filter_map(|word| word.chars().next())
                .map(|c| {
                    let mut s = String::with_capacity(2);
                    s.push(c.to_ascii_uppercase());
                    s.push('.');
                    s
                })
                .collect::<Vec<_>>()
                .join(" ")
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
