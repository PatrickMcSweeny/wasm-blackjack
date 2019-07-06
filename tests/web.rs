//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_blackjack;
use wasm_bindgen_test::*;
use wasm_blackjack::deck::Deck;
use wasm_blackjack::card::Card;
use wasm_blackjack::player::Player;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn card_is_displayed(){
  let mut card = Card::new('H', 12);
  assert_eq!(card.display(), "Q &hearts;".to_string())
}

#[wasm_bindgen_test]
fn face_card_is_ten(){
  let mut card = Card::new('H', 12);
  assert_eq!(card.value(0), 10);
}

#[wasm_bindgen_test]
fn ace_is_eleven(){
  let mut card = Card::new('S', 14);
  assert_eq!(card.value(0), 11);
}

#[wasm_bindgen_test]
fn ace_is_one_when_facing_bust(){
  let mut card = Card::new('S', 14);
  assert_eq!(card.value(11), 1);
}

#[wasm_bindgen_test]
fn deck_is_randomized(){
  let mut deck1 = Deck::new();
  let mut deck2 = Deck::new();
  let mut card1 = deck1.deal();
  let mut card2 = deck2.deal();
  assert_ne!(card1, card2);
}
