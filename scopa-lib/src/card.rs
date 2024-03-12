use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suite {
    Clubs,
    Coins,
    Cups,
    Swords,
}

#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize)]
pub enum CardValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Fante,
    Cavallo,
    Re,
}

impl CardValue {
    pub fn value(&self) -> u8 {
        use CardValue::*;
        match self {
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Fante => 8,
            Cavallo => 9,
            Re => 10,
        }
    }

    pub fn prime(&self) -> u8 {
        use CardValue::*;
        match self {
            Seven => 21,
            Six => 18,
            One => 16,
            Five => 15,
            Four => 14,
            Three => 13,
            Two => 12,
            _ => 10,
        }
    }
}

impl PartialEq for CardValue {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl std::hash::Hash for CardValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value().hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub suite: Suite,
    pub value: CardValue,
}

impl Card {
    pub fn new(suite: Suite, value: CardValue) -> Self {
        Self { suite, value }
    }

    pub fn value(&self) -> u8 {
        self.value.value()
    }

    pub fn prime(&self) -> u8 {
        self.value.prime()
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(40);
        use Suite::*;
        let suites = [Coins, Clubs, Cups, Swords];
        use CardValue::*;
        let values = [One, Two, Three, Four, Five, Six, Seven, Fante, Cavallo, Re];
        for suite in suites {
            for value in values {
                cards.push(Card { suite, value });
            }
        }
        Self { cards }
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    // Deal last 3 cards from the deck
    pub fn deal_hand(&mut self) -> [Card; 3] {
        // It is safe to unwrap because:
        // - scopa deck has 40 cards
        // - on the first turn we place 4 on the table and deal 3 to each of two players, so 30
        // cards left
        // - each turn after that we deal 3 cards to each of two players - 6 cards per turn
        // - 30 is a multiple of 6
        [
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
        ]
        // Looks much cooler but there are no reasons to return Vec
        // let start = self.cards.len() - 3;
        // self.cards.drain(start..).collect()
    }

    // Deal last 4 cards from the deck
    pub fn place_table(&mut self) -> [Card; 4] {
        // It is safe to unwrap because we are placing cards on the table at the beginning of a game,
        // when deck has 40 cards in it
        [
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
        ]
    }
}
