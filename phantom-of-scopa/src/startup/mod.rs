mod components;
mod systems;

use crate::{despawn_screen, AppState};
use components::*;
use systems::*;

use bevy::prelude::*;

pub fn startup_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::StartUp), setup_startup)
        .add_systems(
            Update,
            handle_username_input.run_if(in_state(AppState::StartUp)),
        )
        .add_systems(OnExit(AppState::StartUp), despawn_screen::<StartUpUIRoot>);
}
