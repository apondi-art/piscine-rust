use std::io;
use std::io::Write;

fn main() {
    let mut trial = 0;
    let answer = "The letter e";
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    
    loop {
        println!("{}", riddle);
        trial += 1;
        
        print!("Enter your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let normalized_guess = guess.trim().to_lowercase();
        
        if normalized_guess == answer.to_lowercase() {
            println!("Number of trials: {}", trial);
            break;
        }
    }
}