use crate::card::*;
use serde::{Deserialize, Serialize};

pub type PlayerId = u64;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub seat: u8,
    pub name: String,
    pub points: u8,
    pub scopas: u8,
    pub hand: Vec<Card>,
    pub taken: TakenCards,
}

impl Player {
    pub fn new(id: PlayerId, name: &str, seat: u8) -> Self {
        Self {
            id,
            seat,
            name: name.into(),
            points: 0,
            scopas: 0,
            hand: Vec::with_capacity(3),
            taken: TakenCards::default(),
        }
    }

    fn reset(&mut self) {
        self.points = 0;
        self.scopas = 0;
        self.hand.clear();
        self.taken.clear();
    }

    fn new_hand(&mut self, hand: &[Card; 3]) {
        self.hand = hand.into();
    }

    pub fn take_cards(&mut self, take: Vec<Card>) {
        for card in take {
            self.taken.take_card(card);
        }
    }

    pub fn results(&self) -> Results {
        let takes = self.taken.count();
        let count_of_coins = self.taken.coins.len();
        let seven_of_coins = self.taken.coins.contains(&Card {
            suite: Suite::Coins,
            value: CardValue::Seven,
        });
        let primes = self.taken.primes();
        Results {
            takes: takes as u8,
            count_of_coins: count_of_coins as u8,
            seven_of_coins,
            primes,
            scopas: self.scopas,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Results {
    pub takes: u8,
    pub count_of_coins: u8,
    pub seven_of_coins: bool,
    pub primes: u8,
    pub scopas: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Points {
    pub id: PlayerId,
    pub points: u8,
    pub details: Results,
}
