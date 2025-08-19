use crate::card::{Card, Suit, Rank};

pub struct CardDeck {
    pub cards: Vec<Card>
}

impl CardDeck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in Suit::all().iter() {
            for rank in Rank::all().iter() {
                cards.push(Card::new(*rank, *suit));
            }
        }

        Self { cards }
    }

    // Manual implementation of swap for learning
    pub fn swap(&mut self, a: usize, b: usize) {
        // temporarily remmoves one element
        let temp = self.cards.remove(a);

        // insert replacing element
        let replacement = if a < b {
            self.cards.remove(b - 1)
        } else {
            self.cards.remove(b)
        };

        self.cards.insert(a, replacement);
        self.cards.insert(b, temp);
    }
}
