#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Suit {
    pub fn random() -> Suit {
        match rand::random::<u8>() % 4 {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => Suit::Heart, // default to Heart if invalid
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        match rand::random::<u8>() % 13 + 1 {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n @ 2..=10 => Rank::Number(n),
            _ => Rank::Ace, // default to Ace if invalid
        }
    }
}

pub fn winner_card(card: &Card) -> bool {
    // The winning card is Ace of Spades
    card.suit == Suit::Spade && card.rank == Rank::Ace
}