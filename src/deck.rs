extern crate rand;

use crate::card::Card;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Deck {
    cards: Vec<Card>,
}

#[wasm_bindgen]
impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        let suits = vec!['D', 'H', 'S', 'C'];
        let mut rng = thread_rng();
        for suit in suits {
            for n in 2..15 {
                cards.push(Card::new(suit, n));
            }
        }

        rng.shuffle(&mut cards);
        Deck { cards: cards }
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
