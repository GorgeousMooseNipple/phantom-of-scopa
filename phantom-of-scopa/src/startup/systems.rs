use crate::config::{Config, CONFIG_PATH};
use crate::error::Result;
use crate::popups::{PopUpEvent, PopUpLocation};
use crate::styles::*;
use crate::AppState;

use super::components::*;

use bevy::prelude::*;
use bevy_simple_text_input::{TextInputBundle, TextInputSubmitEvent};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

pub fn setup_startup(
    mut commands: Commands,
    mut popup_event: EventWriter<PopUpEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    let root = commands
        .spawn((
            StartUpUIRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: DEFAULT_BG.into(),
                ..default()
            },
        ))
        .id();

    // Title
    commands
        .spawn((
            StartUpUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((StartUpUI, game_title(&asset_server)));
        })
        .set_parent(root);

    commands
        .spawn((
            StartUpUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(80.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .set_parent(root);

    match load_config() {
        Ok(Some(config)) => {
            commands.insert_resource(config);
            app_state.set(AppState::MainMenu);
        }
        Ok(None) => {
            commands
                .spawn((
                    StartUpUI,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        StartUpUI,
                        TextBundle {
                            style: Style {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            text: default_text("Input username:", &asset_server),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        StartUpUI,
                        NodeBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                border: UiRect::all(Val::Px(5.0)),
                                padding: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            border_color: INACTIVE_UI.into(),
                            ..default()
                        },
                        TextInputBundle::default().with_text_style(TextStyle {
                            font: asset_server.load(DEFAULT_FONT),
                            font_size: INPUT_FONT_SIZE,
                            color: Color::BLACK,
                        }),
                    ));
                })
                .set_parent(root);
        }
        Err(e) => {
            popup_event.send(error_popup(e.to_string()));
        }
    }
}

fn load_config() -> Result<Option<Config>> {
    if Path::new(CONFIG_PATH).exists() {
        let config_as_string = read_to_string(CONFIG_PATH)?;
        let config: Config = toml::from_str(&config_as_string)?;
        Ok(Some(config))
    } else {
        Ok(None)
    }
}

fn create_default_config(username: String) -> Result<Config> {
    let mut file = File::create(CONFIG_PATH)?;
    let default_config = Config::default_with_username(username);
    let config_string = toml::to_string(&default_config)?;
    file.write_all(config_string.as_bytes())?;
    Ok(default_config)
}

pub fn handle_username_input(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut text_input_events: EventReader<TextInputSubmitEvent>,
    mut popup_event: EventWriter<PopUpEvent>,
) {
    for input in text_input_events.read() {
        match create_default_config(input.value.clone()) {
            Ok(config) => {
                commands.insert_resource(config);
                app_state.set(AppState::MainMenu);
            }
            Err(e) => {
                popup_event.send(error_popup(e.to_string()));
            }
        }
    }
}
