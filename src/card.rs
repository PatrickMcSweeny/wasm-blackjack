use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    suit: char,
    pub rank_number: u8,
}

#[wasm_bindgen]
impl Card {
    pub fn new(suit: char, rank_number: u8) -> Card {
        Card {
            rank_number: rank_number,
            suit: suit,
        }
    }

    pub fn display(&mut self) -> String {
        format!("{} {}", self.rank(), self.html_suit())
    }

    pub fn value(&mut self, score: u8) -> u8 {
        match self.rank_number {
            2...10 => self.rank_number,
            11...13 => 10,
            14 => {
                if score + 11 > 21 {
                    1
                } else {
                    11
                }
            }
            _ => 0,
        }
    }

    fn rank(&mut self) -> String {
        match self.rank_number {
            2...10 => format!("{}", self.rank_number),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => "".to_string(),
        }
    }

    fn html_suit(&mut self) -> String {
        match self.suit {
            'H' => "&hearts;".to_string(),
            'D' => "&diams;".to_string(),
            'C' => "&clubs;".to_string(),
            'S' => "&spades;".to_string(),
            _ => "".to_string(),
        }
    }
}
