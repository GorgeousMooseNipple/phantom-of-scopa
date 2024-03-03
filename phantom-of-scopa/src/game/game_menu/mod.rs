mod components;
mod systems;

use crate::game::{InGameState, ScopaState};
use crate::{despawn_screen, AppState};
use bevy::prelude::*;
use components::InGameMenuUI;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct InGameMenuSet;

pub fn game_menu_plugin(app: &mut App) {
    app.configure_sets(
        Update,
        InGameMenuSet.run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Menu))),
    )
    .add_systems(OnEnter(InGameState::Menu), setup_menu)
    .add_systems(Update, (highlight_buttons).in_set(InGameMenuSet))
    .add_systems(OnExit(InGameState::Menu), despawn_screen::<InGameMenuUI>);
}
