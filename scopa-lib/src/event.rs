use crate::player::{PlayerId, Points};
use crate::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    PlayerConnected {
        id: PlayerId,
        name: String,
    },
    PlayerDisconnected {
        id: PlayerId,
        name: String,
    },
    StartRound {
        active_player: PlayerId,
    },
    EndRound {
        points: [Points; 2],
    },
    PlayerWon {
        id: PlayerId,
    },
    DealHand {
        hand: [Card; 3],
    },
    PlaceTable {
        table: [Card; 4],
    },
    PutCard {
        id: PlayerId,
        card: Card,
    },
    TakeCards {
        id: PlayerId,
        take: Vec<Card>,
        with: Card,
    },
}
