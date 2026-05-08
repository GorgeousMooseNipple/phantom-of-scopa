#![allow(unused)]
use crate::popups::{PopUpEvent, PopUpLocation};

use bevy::prelude::*;

// Locations
pub const PLAYER_HAND_X: f32 = 0.0;
pub const PLAYER_HAND_Y: f32 = -182.0;
pub const TABLE_X: f32 = 196.0;
pub const TABLE_Y: f32 = 124.0;
pub const PUT_BUTTON_X: f32 = 328.;
pub const PUT_BUTTON_Y: f32 = -27.;
pub const TAKE_BUTTON_X: f32 = 328.;
pub const TAKE_BUTTON_Y: f32 = 28.;
pub const PLAYER_TAKEN_PILE_X: f32 = -317.;
pub const PLAYER_TAKEN_PILE_Y: f32 = -157.5;
pub const OPPONENT_TAKEN_PILE_X: f32 = 317.;
pub const OPPONENT_TAKEN_PILE_Y: f32 = 157.5;
pub const PLAYER_NAME_X: f32 = 337.5;
pub const PLAYER_NAME_Y: f32 = -191.0;
pub const OPPONENT_NAME_X: f32 = -333.5;
pub const OPPONENT_NAME_Y: f32 = 152.0;

// Sizes
pub const HAND_WIDTH: f32 = 282.0;
pub const HAND_HEIGHT: f32 = 113.0;
pub const HAND_CARDS_SPACING: f32 = 24.0;
pub const CARD_W: f32 = 71.0;
pub const CARD_H: f32 = 110.0;
pub const CARD_SLOT_W: f32 = 78.0;
pub const CARD_SLOT_H: f32 = 114.0;
pub const TABLE_SLOT_W: f32 = 77.0;
pub const TABLE_SLOT_H: f32 = 111.0;
pub const CARD_SLOT_WIDTH: f32 = 78.0;
pub const CARD_SLOT_HEIGHT: f32 = 113.0;
pub const TABLE_BORDER_W: f32 = 4.0;
pub const TABLE_WIDTH: f32 = 410.0;
pub const TABLE_HEIGHT: f32 = 234.0;
pub const TABLE_SLOT_WIDTH: f32 = 77.0;
pub const TABLE_SLOT_HEIGHT: f32 = 111.0;
pub const BUTTON_WIDTH: f32 = 120.0;
pub const BUTTON_HEIGHT: f32 = 51.0;
pub const NAME_W: f32 = 93.;
pub const NAME_H: f32 = 33.;

// Colors
pub const DEFAULT_BG: Color = Color::rgba(0.11, 0.13, 0.13, 1.0);
pub const TEXT_COLOR: Color = Color::rgba(0.85, 0.82, 0.16, 1.0);
pub const INACTIVE_UI: Color = Color::rgba(0.60, 0.66, 0.24, 1.0);
pub const HOVERED_INACTIVE_UI: Color = Color::rgba(0.76, 0.80, 0.50, 1.0);
pub const HOVERED_UI: Color = Color::rgba(0.85, 0.82, 0.16, 1.0);
pub const SELECTED_UI: Color = Color::rgba(0.85, 0.82, 0.16, 1.0);
// pub const SELECTED_UI: Color = Color::rgba(0.38, 0.02, 0.03, 1.0);
pub const HOVERED_SELECTED_UI: Color = Color::rgba(0.77, 0.74, 0.10, 1.0);
pub const DEFAULT_TINT: Color = Color::WHITE;
pub const SELECTION_TINT: Color = Color::rgba(1.0, 0.3, 0.3, 1.0);

// Fonts
pub const DEFAULT_FONT: &str = "fonts/DroidSerif-Regular.ttf";
pub const DEFAULT_FONT_SIZE: f32 = 17.0;
pub const INPUT_FONT_SIZE: f32 = 20.0;
pub const TITLE_FONT_SIZE: f32 = 24.0;

// Settings
pub const DEFAULT_VOLUME: f32 = 0.1;

// Layers
pub const TABLE_LAYER: f32 = 1.0;
pub const ON_TABLE_LAYER: f32 = 2.0;
pub const AREA_LAYER: f32 = 100.0;

#[derive(Resource)]
pub struct DefaultFont {
    pub font: Handle<Font>,
}

pub fn default_text_style(font: &Handle<Font>) -> TextStyle {
    TextStyle {
        font: font.clone(),
        font_size: DEFAULT_FONT_SIZE,
        color: TEXT_COLOR,
    }
}

pub fn default_text(text: &str, font: &Handle<Font>) -> Text {
    Text::from_section(text, default_text_style(font))
}

pub fn game_title(font: &Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            "PHANTOM OF SCOPA",
            TextStyle {
                font: font.clone(),
                font_size: TITLE_FONT_SIZE,
                color: TEXT_COLOR,
            },
        ),
        ..default()
    }
}

pub fn default_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
            border: UiRect::all(Val::Px(2.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        background_color: DEFAULT_BG.into(),
        border_color: INACTIVE_UI.into(),
        ..default()
    }
}

pub fn error_popup(msg: String) -> PopUpEvent {
    PopUpEvent {
        text: msg,
        duration: 15.0,
        location: PopUpLocation::Center,
        height: Val::Percent(40.0),
        ..default()
    }
}
