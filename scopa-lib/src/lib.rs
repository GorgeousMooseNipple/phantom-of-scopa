pub mod card;

use card::*;
use serde::{Deserialize, Serialize};

pub type PlayerId = u64;

#[derive(Debug)]
pub struct TakenCards {
    coins: Vec<Card>,
    clubs: Vec<Card>,
    cups: Vec<Card>,
    swords: Vec<Card>,
}

impl Default for TakenCards {
    fn default() -> Self {
        Self {
            coins: Vec::with_capacity(10),
            clubs: Vec::with_capacity(10),
            cups: Vec::with_capacity(10),
            swords: Vec::with_capacity(10),
        }
    }
}

impl TakenCards {
    fn clear(&mut self) {
        self.coins.clear();
        self.clubs.clear();
        self.cups.clear();
        self.swords.clear();
    }

    fn count(&self) -> usize {
        self.coins.len() + self.clubs.len() + self.cups.len() + self.swords.len()
    }

    fn primes(&self) -> u8 {
        self.coins.iter().max().map_or(0, |c| c.prime())
            + self.clubs.iter().max().map_or(0, |c| c.prime())
            + self.cups.iter().max().map_or(0, |c| c.prime())
            + self.swords.iter().max().map_or(0, |c| c.prime())
    }
}

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    name: String,
    points: u8,
    scopas: u8,
    hand: Vec<Card>,
    taken: TakenCards,
}

impl Player {
    fn new(id: PlayerId, name: String) -> Self {
        Self {
            id,
            name,
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

    fn take_cards(&mut self, take: Vec<Card>) {
        for card in take {
            use Suite::*;
            let put_into = match card.suite {
                Coins => &mut self.taken.coins,
                Clubs => &mut self.taken.clubs,
                Cups => &mut self.taken.cups,
                Swords => &mut self.taken.swords,
            };
            put_into.push(card);
        }
    }

    fn results(&self) -> Results {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Results {
    takes: u8,
    count_of_coins: u8,
    seven_of_coins: bool,
    primes: u8,
    scopas: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Points {
    id: PlayerId,
    points: u8,
    details: Results,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    PlayerConnected { id: PlayerId, name: String },
    PlayerDisconnected { id: PlayerId, name: String },
    StartRound { active_player: PlayerId },
    EndRound { points: [Points; 2] },
    PlayerWon { id: PlayerId },
    DealHand { hand: [Card; 3] },
    PlaceTable { table: [Card; 4] },
    PutCard { card: Card },
    TakeCards { take: Vec<Card>, with: Card },
}

#[derive(Debug)]
pub struct ScopaGame {
    players: Vec<Player>,
    deck: Deck,
    table: Table,
    active_player: PlayerId,
    took_last: usize,
}

impl Default for ScopaGame {
    fn default() -> Self {
        Self {
            players: Vec::with_capacity(2),
            deck: Deck::default(),
            table: Table::default(),
            active_player: u64::default(),
            took_last: usize::default(),
        }
    }
}

impl ScopaGame {
    fn validate(&self, event: &GameEvent) -> bool {
        unimplemented!();
    }

    fn consume(&mut self, event: &GameEvent) {
        unimplemented!()
    }
}
