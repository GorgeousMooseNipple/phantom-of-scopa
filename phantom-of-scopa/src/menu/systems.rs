use crate::error::Result;
use crate::popups::{PopUpEvent, PopUpLocation};
use crate::styles::*;
use crate::AppState;

use super::components::*;

use bevy::prelude::*;
use bevy_simple_text_input::{TextInputBundle, TextInputSubmitEvent, TextInputValue};

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = commands
        .spawn((
            MainMenuUIRoot,
            MainMenuUI,
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
            MainMenuUI,
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
            parent.spawn((MainMenuUI, game_title(&asset_server)));
        })
        .set_parent(root);

    // Fill Space
    commands
        .spawn((
            MainMenuUI,
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

    // Input
    commands
        .spawn((
            MainMenuUI,
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
                MainMenuUI,
                TextBundle {
                    text: default_text(
                        "Input server ip and port in a format ip:port",
                        &asset_server,
                    ),
                    ..default()
                },
            ));
            parent.spawn((
                MainMenuUI,
                NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::vertical(Val::Px(10.0)),
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
            parent
                .spawn((MainMenuUI, ConnectButton, default_button()))
                .with_children(|button| {
                    button.spawn((
                        MainMenuUI,
                        TextBundle {
                            text: default_text("Connect", &asset_server),
                            ..default()
                        },
                    ));
                });
        })
        .set_parent(root);
}

pub fn connect_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<ConnectButton>)>,
    text_input_q: Query<&TextInputValue, With<MainMenuUI>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut popup_events: EventWriter<PopUpEvent>,
) {
    if let Ok(Interaction::Pressed) = interactions.get_single() {
        app_state.set(AppState::InGame);
    }
    // if let Ok(input) = text_input_q.get_single() {
    //     match connect(input.0.as_str()) {
    //         Ok(_) => {
    //             todo!("Connected successfully, so change state to AppState::InGame");
    //         }
    //         Err(e) => {
    //             popup_events.send(PopUpEvent {
    //                 text: e.to_string(),
    //                 duration: 15.0,
    //                 location: PopUpLocation::Center,
    //                 height: Val::Percent(40.0),
    //                 ..default()
    //             });
    //         }
    //     }
    // }
}

pub fn handle_connection_input(
    mut text_input_events: EventReader<TextInputSubmitEvent>,
    mut popup_events: EventWriter<PopUpEvent>,
) {
    for input in text_input_events.read() {
        match connect(input.value.as_str()) {
            Ok(_) => {
                todo!("Connected successfully, so change state to AppState::InGame");
            }
            Err(e) => {
                popup_events.send(PopUpEvent {
                    text: e.to_string(),
                    duration: 15.0,
                    location: PopUpLocation::Center,
                    height: Val::Percent(40.0),
                    ..default()
                });
            }
        }
    }
}

fn connect(connection_string: &str) -> Result<()> {
    todo!("Try to connect to server");
}
