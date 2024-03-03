mod components;
mod systems;

use crate::game::{InGameState, ScopaState};
use crate::{despawn_screen, AppState};
use bevy::prelude::*;
use systems::*;

pub fn game_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::Menu), setup_menu)
        .add_systems(OnExit(InGameState::Menu), teardown_menu);
}
