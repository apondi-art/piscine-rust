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
        // Simple pseudo-random implementation
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        Suit::translate((seed % 4 + 1) as u8)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => Suit::Heart,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        // Simple pseudo-random implementation
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        Rank::translate((seed % 13 + 1) as u8)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Ace,
        }
    }
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}