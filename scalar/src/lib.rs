pub fn sum(a: u8, b: u8) -> u8 {
    a+b

}

pub fn diff(a: i16, b: i16) -> i16 {
    a-b

}

pub fn pro(a: i8, b: i8) -> i8{
    a*b

}

pub fn quo(a: i32, b:i32) -> i32 {
    a/b

}

pub fn rem(a: i32, b: i32) -> i32{
    a%b

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use super :: *;
        let result = sum(2, 2);
        assert_eq!(result, 4);
        let val = diff(2, 2);
        assert_eq!(val, 0);
        let mult = pro(2, 2);
        assert_eq!(mult, 4);
        let div = quo(8, 2);
        assert_eq!(div, 4);
        let modu = rem(4, 2);
        assert_eq!(modu, 0);
    }
}
