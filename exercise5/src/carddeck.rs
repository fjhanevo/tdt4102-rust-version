use rand::Rng;
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
    fn swap(&mut self, a: usize, b: usize) {
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

    pub fn print(&self) {
        for c in self.cards.iter() {
            println!("{}", c.to_string());
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        let len = self.cards.len();
        for _ in 0..len {
            let a = rng.random_range(0..len-1);
            let b = rng.random_range(0..len-1);
            self.swap(a,b);
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        Some(self.cards.pop().expect("Deck is empty."))
    }
}
