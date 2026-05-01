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

#[derive(Message)]
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
    mut popup_events: MessageReader<PopUpEvent>,
    time: Res<Time>,
    default_font: Res<DefaultFont>,
) {
    for event in popup_events.read() {
        // Clear popups that are already present on the same part of the screen
        for (id, popup) in &popups_query {
            if popup.location == event.location {
                commands.entity(id).despawn();
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
                    expiration_time: time.elapsed_secs_f64() + event.duration,
                    location: event.location,
                },
                Node {
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
                BackgroundColor(DEFAULT_BG),
            ))
            .with_children(|parent| {
                parent.spawn((
                    PopUpText,
                    default_text(event.text.as_str(), &default_font.font),
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
        if time.elapsed_secs_f64() > message.expiration_time {
            commands.entity(id).despawn();
        }
    }
}
