mod card;
mod carddeck;
mod blackjack;

use blackjack::Blackjack;

fn main() {
    let mut blackjack = Blackjack::new();
    blackjack.play_game();
}
