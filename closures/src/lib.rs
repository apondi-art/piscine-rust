
pub fn first_fifty_even_square() -> Vec<i32> {
(1..).map(|x| x*2)
.map(|c| c*c)
.take(50)
.collect()
}