mod components;
mod systems;

use crate::{despawn_screen, AppState};
use components::*;
use systems::*;

use bevy::prelude::*;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(
            Update,
            (connect_button).run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<MainMenuUIRoot>);
}
