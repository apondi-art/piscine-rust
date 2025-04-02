use std::io;
use std::io::Write;

fn main() {
    let mut trial = 0;
    let answer = "The letter e"; // Using &str instead of String for simplicity
    
   

    loop {
        trial += 1;
        
        print!("Enter your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
         // Store the original input (trimmed) for display
         let original_guess = guess.trim().to_string();
         // Create lowercase version for comparison
         let normalized_guess = original_guess.to_lowercase();
        
        if normalized_guess == answer.to_lowercase() {
            println!("{}", answer);
            println!("Number of trials: {}", trial);
            break;
        }
        
        // Print the riddle again for the next attempt
        println!("\nI am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        println!("{original_guess}");
    }
}