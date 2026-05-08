#![allow(clippy::type_complexity)]
mod components;
mod game_menu;
mod resources;
mod systems;

use bevy::prelude::*;
use components::InGameComponent;

use crate::{despawn_screen, AppState};
use systems::*;

#[derive(States, Hash, Debug, PartialEq, Eq, Copy, Clone, Default)]
enum GameState {
    Menu,
    #[default]
    Playing,
}

#[allow(unused)]
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
struct GameInteractionSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct PlayerTurnSet;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(game_menu::game_menu_plugin)
        .init_state::<GameState>()
        .init_state::<ScopaState>()
        .add_event::<DrawEvent>()
        .add_event::<PlayAudio>()
        .configure_sets(
            Update,
            (
                InGameSet.run_if(in_state(AppState::InGame)),
                GameInteractionSet
                    .in_set(InGameSet)
                    .run_if(in_state(GameState::Playing)),
                PlayerTurnSet
                    .in_set(GameInteractionSet)
                    .run_if(in_state(ScopaState::PlayerTurn)),
                DragAndDrop.in_set(PlayerTurnSet),
            ),
        )
        .add_systems(OnEnter(AppState::InGame), game_setup)
        .add_systems(Update, spawn_audio)
        .add_systems(Update, debug_areas)
        .add_systems(Update, attach_overlays)
        .add_systems(
            Update,
            (show_overlay_on_cursor_over, hide_overlay_on_cursor_out).in_set(GameInteractionSet),
        )
        .add_systems(Update, (on_draw_hand).in_set(InGameSet))
        .add_systems(OnExit(GameState::Playing), hide_overlays)
        .add_systems(Update, (toggle_in_game_menu).in_set(InGameSet))
        .add_systems(OnExit(AppState::InGame), despawn_screen::<InGameComponent>);
}
