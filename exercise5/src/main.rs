mod card;

use card::{Card, Rank, Suit};

fn main() {
    let c = Card::new(Rank::Ace, Suit::Spades);
    println!("{}", c.to_string());
}
