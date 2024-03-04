#![allow(clippy::type_complexity)]
mod components;
mod game_menu;
mod resources;
mod systems;

use bevy::prelude::*;
use components::InGameComponent;

use super::{despawn_screen, AppState};
use systems::*;

#[derive(States, Hash, Debug, PartialEq, Eq, Copy, Clone, Default)]
enum InGameState {
    Menu,
    #[default]
    Playing,
}

#[derive(States, Hash, Debug, PartialEq, Eq, Copy, Clone, Default)]
enum ScopaState {
    Limbo,
    #[default]
    PlayerTurn,
    OpponentTurn,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct DragAndDrop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct InGameSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct PlayerSet;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(game_menu::game_menu_plugin)
        .init_state::<InGameState>()
        .init_state::<ScopaState>()
        .add_event::<GameEvent>()
        .configure_sets(
            Update,
            (
                InGameSet.run_if(in_state(AppState::InGame)),
                PlayerSet.in_set(InGameSet).run_if(
                    in_state(InGameState::Playing).and_then(in_state(ScopaState::PlayerTurn)),
                ),
                DragAndDrop.in_set(PlayerSet),
            ),
        )
        .add_systems(OnEnter(AppState::InGame), game_setup)
        .add_systems(Update, (toggle_in_game_menu, update_hand).in_set(InGameSet))
        .add_systems(
            Update,
            (
                button_highlights,
                take_button_pressed,
                select_hand_card,
                select_table_card,
                update_selected_cards
                    .after(select_hand_card)
                    .after(select_table_card),
                put_button_pressed,
            )
                .in_set(PlayerSet),
        )
        .add_systems(
            Update,
            (drag_start, update_drag_cursor, drop_in, highlight_on_drag).in_set(DragAndDrop),
        )
        .add_systems(OnExit(AppState::InGame), despawn_screen::<InGameComponent>);
}
