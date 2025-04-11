pub fn stars(n: u32) -> String {
    let  num:u32 = 2u32.pow(n);
    "*".repeat(num as usize)
}