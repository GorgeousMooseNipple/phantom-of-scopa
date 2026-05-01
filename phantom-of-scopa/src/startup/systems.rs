use crate::config::Config;
use crate::popups::PopUpEvent;
use crate::styles::*;
use crate::AppState;

use super::components::*;

use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputSubmitMessage, TextInputTextColor, TextInputTextFont,
};

pub fn setup_startup(
    mut commands: Commands,
    mut popup_event: MessageWriter<PopUpEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    default_font: Res<DefaultFont>,
) {
    let root = commands
        .spawn((
            StartUpUIRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(DEFAULT_BG),
        ))
        .id();

    // Title
    commands
        .spawn((
            StartUpUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ChildOf(root),
        ))
        .with_children(|parent| {
            parent.spawn((StartUpUI, game_title(&default_font.font)));
        });

    commands.spawn((
        StartUpUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(80.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ChildOf(root),
    ));

    match Config::load_config() {
        Ok(Some(config)) => {
            commands.insert_resource(config);
            app_state.set(AppState::MainMenu);
        }
        Ok(None) => {
            commands
                .spawn((
                    StartUpUI,
                    Node {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ChildOf(root),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        StartUpUI,
                        default_text("Input username:", &default_font.font),
                    ));
                    parent.spawn((
                        StartUpUI,
                        Node {
                            width: Val::Px(200.0),
                            border: UiRect::all(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        BorderColor::all(INACTIVE_UI),
                        TextInput::default(),
                        TextInputTextFont(TextFont {
                            font: default_font.font.clone(),
                            font_size: DEFAULT_FONT_SIZE,
                            ..default()
                        }),
                        TextInputTextColor(TextColor(TEXT_COLOR)),
                    ));
                });
        }
        Err(e) => {
            popup_event.write(error_popup(e.to_string()));
        }
    }
}

pub fn handle_username_input(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut text_input_events: MessageReader<TextInputSubmitMessage>,
    mut popup_event: MessageWriter<PopUpEvent>,
) {
    for input in text_input_events.read() {
        match Config::create_init_config(input.value.clone()) {
            Ok(config) => {
                commands.insert_resource(config);
                app_state.set(AppState::MainMenu);
            }
            Err(e) => {
                popup_event.write(error_popup(e.to_string()));
            }
        }
    }
}
