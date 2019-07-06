use crate::card::Card;
use crate::deck::Deck;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Player {
    cards: Vec<Card>,
    pub score: u8,
}

#[wasm_bindgen]
impl Player {
    pub fn new(deck: &mut Deck) -> Player {
        let mut player = Player {
            cards: Vec::new(),
            score: 0,
        };
        for _ in 1..3 {
            player.hit(deck);
        }
        player
    }

    pub fn bust(&mut self) -> bool {
        self.score > 21
    }

    pub fn hit(&mut self, deck: &mut Deck) {
        if let Some(card) = deck.deal() {
            self.cards.push(card);
        }
        self.update_score();
    }

    pub fn hand(&mut self) -> String {
        let mut display = String::from("");
        for card in &mut self.cards {
            display = format!("{} {}", display, card.display())
        }
        display
    }

    pub fn dealer_round(&mut self, deck: &mut Deck) {
        if !self.stand() {
            self.hit(deck);
        }
    }

    pub fn stand(&mut self) -> bool {
        self.score > 16
    }

    fn update_score(&mut self) {
        self.score = 0;
        let cards_by_rank = &mut self.cards;
        cards_by_rank.sort_by_key(|card| card.rank_number);
        for card in &mut cards_by_rank.into_iter() {
            self.score += card.value(self.score)
        }
    }
}
