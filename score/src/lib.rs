
fn score(s:&str)-> u64{
   s.chars()
   .filter(|c| c.is_ascii_alphabetic())
   .map(|c| {
    let upper = c.to_ascii_uppercase();
    match upper{
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0, 
    }
   })
   .sum()

}