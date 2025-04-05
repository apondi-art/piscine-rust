use rand::Rng;

// Define the Suit enum with the four possible suits
#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

// Define the Rank enum with variants for Ace, Face cards, and Number cards
#[derive(Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

// Card structure that combines a Suit and Rank
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Suit {
    // Returns a random Suit
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..=4); // Generate a number between 1 and 4
        Suit::translate(value)
    }

    // Converts a number to the corresponding Suit
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => Suit::Heart, // Default to Heart if out of range
        }
    }
}

impl Rank {
    // Returns a random Rank
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..=13); // Generate a number between 1 and 13
        Rank::translate(value)
    }

    // Converts a number to the corresponding Rank
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Ace, // Default to Ace if out of range
        }
    }
}

// Checks if the card is the winning card (Ace of Spades)
pub fn winner_card(card: &Card) -> bool {
    match (&card.rank, &card.suit) {
        (Rank::Ace, Suit::Spade) => true,
        _ => false,
    }
}
