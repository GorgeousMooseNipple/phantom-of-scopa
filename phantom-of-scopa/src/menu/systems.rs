use crate::config::Config;
use crate::error::Result;
use crate::popups::PopUpEvent;
use crate::styles::*;
use crate::AppState;

use super::components::*;

use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputSubmitMessage, TextInputTextColor, TextInputTextFont, TextInputValue,
};

pub fn setup_menu(mut commands: Commands, config: Res<Config>, default_font: Res<DefaultFont>) {
    let root = commands
        .spawn((
            MainMenuUIRoot,
            MainMenuUI,
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
            MainMenuUI,
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
            parent.spawn((MainMenuUI, game_title(&default_font.font)));
        });

    // Fill Space
    commands.spawn((
        MainMenuUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(80.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ChildOf(root),
    ));

    // Input
    commands
        .spawn((
            MainMenuUI,
            ChildOf(root),
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                MainMenuUI,
                default_text(
                    "Input server ip and port in a format 'ip:port'",
                    &default_font.font,
                ),
            ));
            parent.spawn((
                MainMenuUI,
                Node {
                    width: Val::Px(200.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::vertical(Val::Px(10.0)),
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
                TextInputValue(config.connection_str()),
            ));
            parent
                .spawn((MainMenuUI, ConnectButton, default_button()))
                .with_children(|button| {
                    button.spawn((MainMenuUI, default_text("Connect", &default_font.font)));
                });
        });
}

#[allow(unused)]
pub fn connect_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<ConnectButton>)>,
    text_input_q: Query<&TextInputValue, With<MainMenuUI>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut popup_events: MessageWriter<PopUpEvent>,
) {
    for interaction in interactions {
        match *interaction {
            Interaction::Pressed => app_state.set(AppState::InGame),
            _ => {}
        }
    }
    // if let Ok(input) = text_input_q.get_single() {
    //     match connect(input.0.as_str()) {
    //         Ok(_) => {
    //             todo!("Connected successfully, so change state to AppState::InGame");
    //         }
    //         Err(e) => {
    //             popup_events.send(error_popup(e.to_string()));
    //         }
    //     }
    // }
}

pub fn handle_connection_input(
    mut text_input_events: MessageReader<TextInputSubmitMessage>,
    mut popup_events: MessageWriter<PopUpEvent>,
) {
    for input in text_input_events.read() {
        match connect(input.value.as_str()) {
            Ok(_) => {
                todo!("Connected successfully, so change state to AppState::InGame");
            }
            Err(e) => {
                popup_events.write(error_popup(e.to_string()));
            }
        }
    }
}

#[allow(unused)]
fn connect(connection_string: &str) -> Result<()> {
    todo!("Try to connect to server");
}
