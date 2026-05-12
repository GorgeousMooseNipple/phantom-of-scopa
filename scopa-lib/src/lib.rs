#![allow(dead_code)]
pub mod card;
pub mod error;
pub mod event;
pub mod player;

use card::*;
use error::{Result, ScopaError};
use event::{ClientEvent, ServerEvent};
use player::*;
use std::collections::HashMap;

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

    pub fn validate(&self, player_id: PlayerId, event: &ClientEvent) -> Result<()> {
        match event {
            ClientEvent::PutCard { card } => {
                if !self.players.contains_key(&player_id) {
                    return Err(ScopaError::Player("Unknown player".into()));
                }
                if self.active_player != player_id {
                    return Err(ScopaError::OutOfTurn);
                }
                // The table is full
                if self.table.len() >= 10 {
                    return Err(ScopaError::PuttingOnFullTable);
                }
                // It is safe to unwrap because we already checked that player is connected
                if !self.players.get(&player_id).unwrap().hand.contains(card) {
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
            ClientEvent::TakeCards { take, with } => {
                if !self.players.contains_key(&player_id) {
                    return Err(ScopaError::Player("Unknown player".into()));
                }
                if self.active_player != player_id {
                    return Err(ScopaError::OutOfTurn);
                }
                if take.is_empty() {
                    return Err(ScopaError::Logic(
                        "You should choose which cards to take".into(),
                    ));
                }
                // It is safe to unwrap because we already checked that player is connected
                if !self.players.get(&player_id).unwrap().hand.contains(with) {
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

    pub fn consume(&mut self, player_id: PlayerId, event: &ClientEvent) -> Result<ServerEvent> {
        #[allow(unused_variables)]
        match event {
            ClientEvent::Connect { name } => {
                let player = Player::new(player_id, name, self.players.len() as u8);
                self.players.insert(player_id, player);
                return Ok(ServerEvent::Welcome {
                    id: player_id,
                    name: name.clone(),
                });
            }
            ClientEvent::Disconnect => {
                if let Some(player) = self.players.remove(&player_id) {
                    Ok(ServerEvent::OpponentLeft {
                        id: player.id,
                        name: player.name,
                        seat: player.seat,
                    })
                } else {
                    Err(ScopaError::Player(format!(
                        "Player with id = {} not found",
                        player_id
                    )))
                }
            }
            ClientEvent::PutCard { card } => {
                if let Some(player) = self.players.get_mut(&player_id) {
                    if let Some(pos) = player.hand.iter().position(|c| c == card) {
                        player.hand.remove(pos);
                    } else {
                        return Err(ScopaError::Player(format!(
                            "Card {} not found in hand of player {:?}",
                            card, player
                        )));
                    }
                    self.table.put_card(*card);
                    Ok(ServerEvent::PutCard {
                        id: player.id,
                        card: *card,
                    })
                } else {
                    Err(ScopaError::Player(format!(
                        "Player with id = {} not found",
                        player_id
                    )))
                }
            }
            ClientEvent::TakeCards { take, with } => {
                // Already checked in validate that player with id exists
                let player = self
                    .players
                    .get_mut(&player_id)
                    .expect("Taking cards: Player not found");
                if let Some(pos) = player.hand.iter().position(|card| card == with) {
                    player.hand.remove(pos);
                } else {
                    return Err(ScopaError::Player(format!(
                        "Card {} not found in hand of player {:?}",
                        with, player
                    )));
                }
                let mut to_take = Vec::new();
                for card in take.iter() {
                    if let Some(card) = self.table.take_card(card) {
                        to_take.push(card);
                    }
                }
                player.take_cards(to_take);
                let scopa = self.table.is_empty();
                Ok(ServerEvent::TakeCards {
                    id: player.id,
                    taken: take.clone(),
                    with: *with,
                    table: self.table.to_vec(),
                    scopa,
                })
            }
        }
    }
}
