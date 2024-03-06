use crate::popups::{PopUpEvent, PopUpLocation};

use bevy::prelude::*;

pub const HAND_WIDTH: f32 = 282.0;
pub const HAND_HEIGHT: f32 = 113.0;
pub const HAND_CARDS_SPACING: f32 = 24.0;
pub const CARD_WIDTH: f32 = 71.0;
pub const CARD_HEIGHT: f32 = 110.0;
pub const PLAYER_HAND_X: f32 = 260.0;
pub const PLAYER_HAND_Y: f32 = 366.0;
pub const CARD_SLOT_WIDTH: f32 = 78.0;
pub const CARD_SLOT_HEIGHT: f32 = 113.0;
pub const IN_GAME_BORDER_WIDTH: f32 = 4.0;
pub const TABLE_WIDTH: f32 = 410.0;
pub const TABLE_HEIGHT: f32 = 234.0;
pub const TABLE_X: f32 = 196.0;
pub const TABLE_Y: f32 = 124.0;
pub const TABLE_SLOT_WIDTH: f32 = 77.0;
pub const TABLE_SLOT_HEIGHT: f32 = 111.0;
pub const BUTTON_WIDTH: f32 = 120.0;
pub const BUTTON_HEIGHT: f32 = 51.0;

pub const DEFAULT_BG: Color = Color::rgba(0.11, 0.13, 0.13, 1.0);
pub const TEXT_COLOR: Color = Color::rgba(0.85, 0.82, 0.16, 1.0);
pub const INACTIVE_UI: Color = Color::rgba(0.60, 0.66, 0.24, 1.0);
pub const HOVERED_INACTIVE_UI: Color = Color::rgba(0.76, 0.80, 0.50, 1.0);
pub const HOVERED_UI: Color = Color::rgba(0.85, 0.82, 0.16, 1.0);
pub const SELECTED_UI: Color = Color::rgba(0.85, 0.82, 0.16, 1.0);
// pub const SELECTED_UI: Color = Color::rgba(0.38, 0.02, 0.03, 1.0);
pub const HOVERED_SELECTED_UI: Color = Color::rgba(0.77, 0.74, 0.10, 1.0);

pub const DEFAULT_FONT: &str = "fonts/DroidSerif-Regular.ttf";
pub const DEFAULT_FONT_SIZE: f32 = 17.0;
pub const INPUT_FONT_SIZE: f32 = 20.0;
pub const TITLE_FONT_SIZE: f32 = 24.0;

pub const DEFAULT_VOLUME: f32 = 0.1;

pub fn default_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(DEFAULT_FONT),
        font_size: DEFAULT_FONT_SIZE,
        color: TEXT_COLOR,
    }
}

pub fn default_text(text: &str, asset_server: &Res<AssetServer>) -> Text {
    Text::from_section(text, default_text_style(asset_server))
}

pub fn game_title(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            "PHANTOM OF SCOPA",
            TextStyle {
                font: asset_server.load(DEFAULT_FONT),
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
