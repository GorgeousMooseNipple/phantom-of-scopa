mod components;
mod systems;

use crate::game::InGameState;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct InGameMenuSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct RootInGameMenuSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct SettingsSet;

#[derive(States, Hash, Debug, PartialEq, Eq, Copy, Clone, Default)]
enum InGameMenuState {
    #[default]
    Closed,
    Root,
    Settings,
}

pub fn game_menu_plugin(app: &mut App) {
    app.init_state::<InGameMenuState>()
        .configure_sets(
            Update,
            (
                InGameMenuSet
                    .run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Menu))),
                RootInGameMenuSet
                    .in_set(InGameMenuSet)
                    .run_if(in_state(InGameMenuState::Root)),
                SettingsSet
                    .in_set(InGameMenuSet)
                    .run_if(in_state(InGameMenuState::Settings)),
            ),
        )
        .add_systems(OnEnter(InGameState::Menu), setup_menu)
        .add_systems(OnEnter(InGameMenuState::Root), create_root_in_game_menu)
        .add_systems(
            OnEnter(InGameMenuState::Settings),
            create_settings_in_game_menu,
        )
        // .add_systems(Update, (highlight_buttons).in_set(InGameMenuSet))
        .add_systems(Update, (open_settings, main_menu).in_set(RootInGameMenuSet))
        .add_systems(
            Update,
            (
                back_to_root,
                highlight_volume_buttons,
                selected_volume_button,
            )
                .in_set(SettingsSet),
        )
        .add_systems(OnExit(InGameMenuState::Settings), despawn_submenu)
        .add_systems(OnExit(InGameMenuState::Root), despawn_submenu)
        .add_systems(OnExit(InGameState::Menu), close_menu);
}
