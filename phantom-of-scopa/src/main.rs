mod error;
mod game;
mod popups;
mod styles;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use popups::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    MainMenu,
    #[default]
    InGame,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_event::<PopUpEvent>()
        .add_systems(Startup, setup)
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
        .add_plugins(game::game_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
