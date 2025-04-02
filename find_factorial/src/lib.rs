pub fn factorial(num: u64) -> u64 {
    let mut result = 1;
    for i in 2..=num {
        result *= i;
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
