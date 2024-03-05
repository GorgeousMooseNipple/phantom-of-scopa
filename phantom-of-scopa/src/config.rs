use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PlayerInfo {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Settings {
    volume: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self { volume: 5 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ConnectionInfo {
    ip: String,
    port: String,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".into(),
            port: "6969".into(),
        }
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    player: PlayerInfo,
    settings: Settings,
    connection: ConnectionInfo,
}
