use crate::styles::*;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PopUpLocation {
    Top,
    #[default]
    Bottom,
    Center,
}

#[derive(Component, Debug)]
pub struct PopUpMessage {
    pub expiration_time: f64,
    pub location: PopUpLocation,
}

#[derive(Component, Debug)]
pub struct PopUpText;

#[derive(Event)]
pub struct PopUpEvent {
    pub text: String,
    pub duration: f64,
    pub location: PopUpLocation,
    pub width: Val,
    pub height: Val,
}

impl Default for PopUpEvent {
    fn default() -> Self {
        Self {
            text: "Pop-up message".into(),
            duration: 2.0,
            location: PopUpLocation::default(),
            width: Val::Percent(60.0),
            height: Val::Percent(15.0),
        }
    }
}

pub fn handle_popups(
    popups_query: Query<(Entity, &PopUpMessage)>,
    mut commands: Commands,
    mut popup_events: EventReader<PopUpEvent>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for event in popup_events.read() {
        // Clear popups that are already present on the same part of the screen
        for (id, popup) in &popups_query {
            if popup.location == event.location {
                commands.entity(id).despawn_recursive();
            }
        }
        let align_popup = match event.location {
            PopUpLocation::Top => AlignSelf::Start,
            PopUpLocation::Bottom => AlignSelf::End,
            PopUpLocation::Center => AlignSelf::Center,
        };
        commands
            .spawn((
                PopUpMessage {
                    expiration_time: time.elapsed_seconds_f64() + event.duration,
                    location: event.location,
                },
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_self: align_popup,
                        justify_self: JustifySelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: event.width,
                        height: event.height,
                        margin: UiRect::vertical(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(DEFAULT_BG),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    PopUpText,
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: event.text.clone(),
                                style: TextStyle {
                                    font: asset_server.load(DEFAULT_FONT),
                                    font_size: DEFAULT_FONT_SIZE,
                                    color: TEXT_COLOR,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    },
                ));
            });
    }
}

pub fn clear_expired_popups(
    popups_query: Query<(Entity, &PopUpMessage)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (id, message) in &popups_query {
        if time.elapsed_seconds_f64() > message.expiration_time {
            commands.entity(id).despawn_recursive();
        }
    }
}
