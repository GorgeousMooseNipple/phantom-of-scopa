use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

pub const CONFIG_PATH: &str = "config.toml";
pub const MIN_VOLUME: f32 = 0.04;
pub const MAX_VOLUME: f32 = 0.40;
pub const VOLUME_STEP: f32 = MAX_VOLUME / 10.0;

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

impl Config {
    pub fn default_with_username(username: String) -> Self {
        Self {
            player: PlayerInfo { name: username },
            settings: Settings::default(),
            connection: ConnectionInfo::default(),
        }
    }

    pub fn volume_as_f32(&self) -> f32 {
        f32::min(
            MAX_VOLUME,
            f32::max(self.settings.volume as f32 * VOLUME_STEP, MIN_VOLUME),
        )
    }

    pub fn set_volume_level(&mut self, volume_level: usize) {
        self.settings.volume = volume_level
    }
}
