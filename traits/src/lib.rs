// Player struct representing a game player with various attributes
#[derive(Debug)]
pub struct Player {
    pub name: String,          // Player's name
    pub strength: f64,         // Player's strength level
    pub score: i32,            // Player's score
    pub money: i32,            // Player's money
    pub weapons: Vec<String>, // Player's list of weapons
}

// Fruit struct representing a type of food with weight
pub struct Fruit {
    pub weight_in_kg: f64, // Weight of the fruit in kilograms
}

// Meat struct representing another type of food with weight and fat content
pub struct Meat {
    pub weight_in_kg: f64, // Weight of the meat in kilograms
    pub fat_content: f64,  // Fat content of the meat
}

// Implementation block for Player
impl Player {
    // Method allowing the player to eat a food item and gain strength
    pub fn eat<T: Food>(&mut self, food: T) {
        self.strength += food.gives(); // Increase strength based on the food's give value
    }
}

use std::fmt;

// Implement Display trait for Player to allow formatted output
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print player name
        writeln!(f, "{}", self.name)?;
        // Print player's strength, score, and money
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            self.strength, self.score, self.money
        )?;
        // Print player's weapons
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

// Trait representing any type of food that gives strength
pub trait Food {
    fn gives(&self) -> f64; // Returns how much strength the food gives
}

// Implement Food trait for Fruit
impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0 // Strength is 4 times the weight
    }
}

// Implement Food trait for Meat
impl Food for Meat {
    fn gives(&self) -> f64 {
        // Strength is 4 times the weight plus additional based on fat content
        self.weight_in_kg * 4.0 + self.weight_in_kg * self.fat_content * 5.0
    }
}