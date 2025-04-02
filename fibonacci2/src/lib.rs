pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    
    let mut a = 0;
    let mut b = 1;
    
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    
    b
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
