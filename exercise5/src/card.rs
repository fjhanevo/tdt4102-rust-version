// src/card.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Clubs, Diamonds, Hearts, Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn blackjack_values(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,

        }
    }
}

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    // Sort of similar to constructor in C++
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn to_string(&self) -> String {
        format!("{} of {}", 
            self.rank.rank_to_string(), self.suit.suit_to_string())
    }
}
