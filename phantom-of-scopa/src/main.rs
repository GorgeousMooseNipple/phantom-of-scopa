mod config;
mod error;
mod game;
mod menu;
mod popups;
mod startup;
mod styles;

use popups::*;
use styles::*;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_simple_text_input::TextInputPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    StartUp,
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_event::<PopUpEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, highlight_buttons)
        .add_systems(Update, (handle_popups, clear_expired_popups))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Phantom of Scopa".into(),
                resolution: (800., 481.).into(),
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                resizable: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TextInputPlugin)
        .add_plugins(startup::startup_plugin)
        .add_plugins(menu::menu_plugin)
        .add_plugins(game::game_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[allow(clippy::type_complexity)]
pub fn highlight_buttons(
    mut buttons_q: Query<(&mut BorderColor, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (mut border_color, interaction) in &mut buttons_q {
        match *interaction {
            Interaction::Hovered => *border_color = HOVERED_UI.into(),
            Interaction::Pressed => *border_color = SELECTED_UI.into(),
            Interaction::None => *border_color = INACTIVE_UI.into(),
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
