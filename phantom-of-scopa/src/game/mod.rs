#![allow(clippy::type_complexity)]
mod components;
mod popups;
mod resources;
mod systems;

use bevy::prelude::*;
use components::InGameComponent;

use super::{despawn_screen, AppState};
use popups::*;
use systems::*;

pub fn game_plugin(app: &mut App) {
    app.add_event::<GameEvent>()
        .add_event::<PopUpEvent>()
        .add_systems(OnEnter(AppState::InGame), game_setup)
        .add_systems(Update, handle_popups)
        .add_systems(Update, clear_expired_popups)
        .add_systems(Update, button_highlights)
        .add_systems(Update, take_button_pressed)
        .add_systems(Update, update_hand)
        .add_systems(Update, select_hand_card)
        .add_systems(Update, select_table_card)
        .add_systems(
            Update,
            update_selected_cards
                .after(select_hand_card)
                .after(select_table_card),
        )
        .add_systems(Update, drag_start)
        .add_systems(Update, update_drag_cursor)
        .add_systems(Update, drop_in)
        .add_systems(Update, highlight_on_drag)
        .add_systems(Update, put_button_pressed)
        .add_systems(OnExit(AppState::InGame), despawn_screen::<InGameComponent>);
}
