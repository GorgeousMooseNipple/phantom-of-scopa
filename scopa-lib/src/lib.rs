#![allow(dead_code)]
pub mod card;
pub mod event;
pub mod player;

use card::*;
use event::GameEvent;
use player::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ScopaError {
    Player(String),
    Logic(String),
    Card(String),
    OutOfTurn,
    PuttingOnFullTable,
}

impl std::fmt::Display for ScopaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ScopaError::*;
        match self {
            Player(msg) => write!(f, "Player error: {}", msg),
            Logic(msg) => write!(f, "Logic error: {}", msg),
            Card(msg) => write!(f, "Card error: {}", msg),
            OutOfTurn => write!(f, "Player made a move out of turn"),
            PuttingOnFullTable => write!(f, "Trying to put a card on a full table"),
        }
    }
}

impl std::error::Error for ScopaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct ScopaGame {
    players: HashMap<PlayerId, Player>,
    deck: Deck,
    table: Table,
    active_player: PlayerId,
    took_last: PlayerId,
}

impl Default for ScopaGame {
    fn default() -> Self {
        Self {
            players: HashMap::with_capacity(2),
            deck: Deck::default(),
            table: Table::default(),
            active_player: PlayerId::default(),
            took_last: PlayerId::default(),
        }
    }
}

impl ScopaGame {
    pub fn new_round(&mut self) {
        let mut deck = Deck::default();
        deck.shuffle();
        self.deck = deck;
        self.table.clear();
    }

    pub fn validate(&self, event: &GameEvent) -> Result<(), ScopaError> {
        match event {
            GameEvent::PlayerConnected { id, .. } => {
                if self.players.contains_key(id) {
                    return Err(ScopaError::Player("Already connected".into()));
                }
            }
            GameEvent::PlayerDisconnected { id, .. } => {
                if !self.players.contains_key(id) {
                    return Err(ScopaError::Player("Unknown player".into()));
                }
            }
            GameEvent::StartRound { active_player } => {
                if !self.players.contains_key(active_player) {
                    return Err(ScopaError::Player("Unknown player".into()));
                }
            }
            GameEvent::PlaceTable { .. } => {
                // Game places 4 cards on the table just once in the beginning of a round, so the table must be
                // empty at this point
                if !self.table.is_empty() {
                    return Err(ScopaError::Logic(
                        "Table should be empty at this stage".into(),
                    ));
                }
            }
            GameEvent::PutCard { id, card } => {
                if !self.players.contains_key(id) {
                    return Err(ScopaError::Player("Unknown player".into()));
                }
                if self.active_player != *id {
                    return Err(ScopaError::OutOfTurn);
                }
                // The table is full
                if self.table.len() >= 10 {
                    return Err(ScopaError::PuttingOnFullTable);
                }
                // It is safe to unwrap because we already checked that player is connected
                if !self.players.get(id).unwrap().hand.contains(card) {
                    return Err(ScopaError::Card(
                        "Card does not exist in player's hand".into(),
                    ));
                }
                // Can't place card on the table if there is a card with the same value - you should
                // take it instead
                if self.table.contains_same_value(card).is_some() {
                    return Err(ScopaError::Logic(
                        "Trying to put card on the table, when you can take a card with it".into(),
                    ));
                }
            }
            GameEvent::TakeCards { id, take, with } => {
                if !self.players.contains_key(id) {
                    return Err(ScopaError::Player("Unknown player".into()));
                }
                if self.active_player != *id {
                    return Err(ScopaError::OutOfTurn);
                }
                if take.is_empty() {
                    return Err(ScopaError::Logic(
                        "You should choose which cards to take".into(),
                    ));
                }
                // It is safe to unwrap because we already checked that player is connected
                if !self.players.get(id).unwrap().hand.contains(with) {
                    return Err(ScopaError::Card(
                        "Card does not exist in player's hand".into(),
                    ));
                }
                if !take.iter().all(|card| self.table.contains(card)) {
                    return Err(ScopaError::Logic(
                        "Trying to take a card which is not present on the table".into(),
                    ));
                }
                if let Some(same_value) = self.table.contains_same_value(with) {
                    if take.len() > 1 || take[0] != *same_value {
                        return Err(ScopaError::Logic(
                                "There is a card with the same value on the table ({same_value}). You should take it with your {with} instead.".into()
                        ));
                    }
                }
                let take_sum: u8 = take.iter().map(|card| card.value()).sum();
                if take_sum != with.value() {
                    return Err(ScopaError::Logic("Trying to take cards with sum value {take_sum} with a card with value of {with.value()}".into()));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn consume(&mut self, _event: &GameEvent) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player_with_cards() -> Player {
        let mut p = Player::new("test");
        p.take_cards(vec![
            Card {
                suite: Suite::Coins,
                value: CardValue::Seven,
            },
            Card {
                suite: Suite::Coins,
                value: CardValue::Six,
            },
            Card {
                suite: Suite::Coins,
                value: CardValue::One,
            },
            Card {
                suite: Suite::Coins,
                value: CardValue::Re,
            },
            Card {
                suite: Suite::Swords,
                value: CardValue::One,
            },
            Card {
                suite: Suite::Swords,
                value: CardValue::Cavallo,
            },
            Card {
                suite: Suite::Cups,
                value: CardValue::One,
            },
            Card {
                suite: Suite::Cups,
                value: CardValue::Six,
            },
            Card {
                suite: Suite::Clubs,
                value: CardValue::Two,
            },
            Card {
                suite: Suite::Clubs,
                value: CardValue::Fante,
            },
            Card {
                suite: Suite::Clubs,
                value: CardValue::Re,
            },
        ]);
        p
    }

    #[test]
    fn take_cards() {
        let p = player_with_cards();
        assert_eq!(p.taken.count(), 11);
        assert_eq!(p.taken.coins.len(), 4);
        assert_eq!(p.taken.swords.len(), 2);
        assert_eq!(p.taken.cups.len(), 2);
        assert_eq!(p.taken.clubs.len(), 3);
    }

    #[test]
    fn get_primes() {
        let p = player_with_cards();
        assert_eq!(p.taken.primes(), 67);
    }

    #[test]
    fn check_results() {
        let p = player_with_cards();
        let r = p.results();
        assert_eq!(r.scopas, 0);
        assert_eq!(r.takes, 11);
        assert_eq!(r.primes, 67);
        assert_eq!(r.count_of_coins, 4);
        assert!(r.seven_of_coins);
    }

    #[test]
    fn compare_cards() {
        let c1 = Card {
            suite: Suite::Coins,
            value: CardValue::Seven,
        };
        let c2 = Card {
            suite: Suite::Coins,
            value: CardValue::Seven,
        };
        let c3 = Card {
            suite: Suite::Swords,
            value: CardValue::Seven,
        };
        let c4 = Card {
            suite: Suite::Coins,
            value: CardValue::Six,
        };
        let c5 = Card {
            suite: Suite::Swords,
            value: CardValue::Six,
        };
        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
        assert_ne!(c1, c4);
        assert_ne!(c1, c5);
    }

    #[test]
    fn table_contains_same_value() {
        let mut table = Table::default();
        use CardValue::*;
        use Suite::*;
        table.put_card(Card {
            suite: Coins,
            value: Two,
        });
        table.put_card(Card {
            suite: Cups,
            value: Five,
        });
        table.put_card(Card {
            suite: Swords,
            value: Two,
        });
        table.put_card(Card {
            suite: Coins,
            value: Four,
        });

        let card = Card {
            suite: Coins,
            value: Five,
        };

        assert_eq!(
            table.contains_same_value(&card),
            Some(&Card {
                suite: Cups,
                value: Five
            })
        );

        let card = Card {
            suite: Coins,
            value: Re,
        };

        assert_eq!(table.contains_same_value(&card), None);
    }
}
