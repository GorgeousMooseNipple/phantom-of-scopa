use crate::player::{PlayerId, Points};
use crate::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientEvent {
    Connect { name: String },
    Disconnect,
    PutCard { card: Card },
    TakeCards { take: Vec<Card>, with: Card },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerEvent {
    Welcome {
        id: PlayerId,
        name: String,
    },
    OpponentJoined {
        name: String,
        seat: u8,
    },
    OpponentLeft {
        id: PlayerId,
        name: String,
        seat: u8,
    },
    PutCard {
        id: PlayerId,
        card: Card,
    },
    TakeCards {
        id: PlayerId,
        taken: Vec<Card>,
        with: Card,
        table: Vec<Card>,
        scopa: bool,
    },
    StartGame {
        active_player: PlayerId,
        hand: [Card; 3],
        table: [Card; 4],
    },
    StartRound {
        active_player: PlayerId,
        hand: [Card; 3],
    },
    NewTurn {
        active_player: PlayerId,
    },
    EndGame {
        points: [Points; 2],
    },
    PlayerWon {
        id: PlayerId,
    },
}
