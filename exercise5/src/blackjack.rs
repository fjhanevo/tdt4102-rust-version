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
    fn is_ace(c: &Card) -> bool {
        c.get_rank() == Rank::Ace
    }

    pub fn get_card_value(c: &Card) -> u8 {
        c.get_rank().blackjack_values()
    }

    pub fn get_hand_score(&self, cards: &[Card]) -> u8 {
        let mut sum = 0;
        let mut aces = 0;

        for card in cards.iter() {
            if Self::is_ace(card) {
                aces += 1;
            } else {
                sum += card.get_rank().blackjack_values();
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

    pub fn ask_player_draw_card() -> bool {
        let mut input = String::new();
        println!("Do you want to draw a card? [y/n]");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        
        if input.trim() == "y" {
            return true
        }
        false



    }

    pub fn draw_player_card(&mut self) {
        if let Some(card) = self.deck.draw_card() {
            self.player_hand.push(card);
            println!("You drew {}", card.to_string());
        } else {
            println!("Deck is empty!");
        }
    }

    pub fn draw_dealer_card(&mut self) {
        if let Some(card) = self.deck.draw_card() {
            self.dealer_hand.push(card);
        } else {
            println!("Deck is empty!");
        }
    }
}



