// src/card.rs

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Clubs, Diamonds, Hearts, Spades,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Two = 2, Three, Four, Five, Six, Seven, 
    Eight, Nine, Ten, Jack, Queen, King, Ace,
}

impl Suit {
    pub fn suit_to_string(&self) -> &'static str {
        match self {
            Suit::Clubs => "Clubs",
            Suit::Diamonds=> "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
        }
    }

    pub fn all() -> [Suit; 4] {
        [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}

impl Rank {
    pub fn rank_to_string(&self) -> &'static str {
        match self {
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        }
    }

    pub fn all() -> [Rank; 13] {
        [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}

pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    // Sort of similar to constructor in C++
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn get_suit(&self) -> &Suit {
        &self.suit
    }

    pub fn get_rank(&self) -> &Rank {
        &self.rank
    }

    pub fn to_string(&self) -> String {
        format!("{} of {}", 
            self.rank.rank_to_string(), self.suit.suit_to_string())
    }
}
