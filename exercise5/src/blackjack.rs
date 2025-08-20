// src/blackjack.rs
use crate::carddeck::CardDeck;
use crate::card::{Card, Rank};
use std::io;

#[derive(Debug)]
pub struct Blackjack {
    deck: CardDeck,
    player_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
    player_hand_sum: u8,
    dealer_hand_sum: u8,
}

impl Blackjack {
    // constructor
    pub fn new() -> Self {
        Blackjack {
            deck: CardDeck::new(),
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            player_hand_sum: 0,
            dealer_hand_sum: 0,
        } 
    }
    fn is_ace(c: &Card) -> bool {
        c.get_rank() == Rank::Ace
    }

    fn get_card_value(c: &Card) -> u8 {
        c.get_rank().blackjack_values()
    }

    fn get_hand_score(&self, cards: &[Card]) -> u8 {
        let mut sum = 0;
        let mut aces = 0;

        for card in cards.iter() {
            if Self::is_ace(card) {
                aces += 1;
            } else {
                sum += Self::get_card_value(&card);
            }
        }

        for _ in 0..aces {
            if sum + 11 <= 21 - (aces - 1) {
                sum += 11;
            } else {
                sum += 1;
            }
        }
        sum
    }

    fn ask_player(s: String) -> bool {
        let mut input = String::new();
        println!("{}", s);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        
        if input.trim() == "y" {
            return true
        }
        false
    }

    fn draw_player_card(&mut self) {
        if let Some(card) = self.deck.draw_card() {
            self.player_hand.push(card);
            println!("You drew {}", card.to_string());
            self.player_hand_sum = self.get_hand_score(&self.player_hand);
        } else {
            println!("Deck is empty!");
        }
    }

    fn draw_dealer_card(&mut self) {
        if let Some(card) = self.deck.draw_card() {
            self.dealer_hand.push(card);
            self.dealer_hand_sum = self.get_hand_score(&self.dealer_hand);
        } else {
            println!("Deck is empty!");
        }
    }

    fn start_round(&mut self) {
        if self.deck.cards.len() < 16 {
            self.deck = CardDeck::new();
            self.deck.shuffle();
        }
        self.player_hand_sum = 0;
        self.dealer_hand_sum = 0;

        // empty both hands
        self.player_hand.clear();
        self.dealer_hand.clear();

        self.draw_player_card();
        self.draw_dealer_card();
        self.draw_player_card();
        self.draw_dealer_card();
    }

    pub fn play_game(&mut self) {
        println!("Welcome to Blackjack!");
        let mut win: bool = false;

        // shuffle twice for fun
        self.deck.shuffle();
        self.deck.shuffle();

        loop {
            self.start_round();

            println!("Player hand sum is: {}", self.player_hand_sum);
            if let Some(card) = self.dealer_hand.get(0) {
                println!("The dealer's first card is: {}", card.to_string());
            }

            while self.player_hand_sum < 21 && Self::ask_player("Do you want to draw a card [y/N]".to_string()) {
                println!("Player draws a card.");
                self.draw_player_card();
                println!("Player hand sum is: {}", self.player_hand_sum);
            }

            while self.dealer_hand_sum < 17 {
                println!("Dealer draws a card.");
                self.draw_dealer_card();
            }

            println!("**********");
            println!("Player's cards: ");
            for card in self.player_hand.iter() {
                println!("{}", card.to_string());
            }
            println!("Dealer's hand sum: {}", self.dealer_hand_sum);

            if self.player_hand_sum > 21 {
                println!("Player bust!");
            } else if self.dealer_hand_sum > 21 {
                println!("Dealer bust!");
                win = true;
            } else if self.player_hand_sum == 21 && self.player_hand.len() == 2 {
                println!("Player got Blackjack!\n");
                win = true;
            } else if self.dealer_hand_sum == 21 && self.dealer_hand.len() == 2 {
                println!("Dealer got Blackjack!\n");
            } else if self.player_hand_sum <= 21 && self.player_hand_sum > self.dealer_hand_sum {
                println!("Player beat dealer!\n");
                win = true;
            } else if self.dealer_hand_sum <= 21 && self.dealer_hand_sum > self.player_hand_sum {
                println!("Dealer beat player!\n");
            }

            if win {
                println!("You won!");
            } else {
                println!("You lost!");
            }
            if !Self::ask_player("Play again? [y/N]".to_string()) {
                break;
            }
        }
    }
}



