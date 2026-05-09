mod config;
mod error;
mod events;
mod game;
mod menu;
mod popups;
mod resources;
mod startup;
mod styles;

use config::Config;
use popups::*;
use resources::*;
use styles::*;

use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_simple_text_input::TextInputPlugin;
use bevy_tweening::TweeningPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    StartUp,
    MainMenu,
    InGame,
}

fn main() {
    setup_logging();

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
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
            })
            .disable::<bevy::log::LogPlugin>(),
    )
    .add_plugins(DefaultPickingPlugins)
    .add_plugins(TextInputPlugin)
    .add_plugins(TweeningPlugin);

    setup_resources(&mut app);

    app.init_state::<AppState>()
        .add_event::<PopUpEvent>()
        .add_event::<events::PlayAudio>()
        .add_systems(Startup, setup)
        .add_systems(Update, highlight_buttons)
        .add_systems(Update, spawn_audio)
        .add_systems(Update, (handle_popups, clear_expired_popups))
        .add_plugins(startup::startup_plugin)
        .add_plugins(menu::menu_plugin)
        .add_plugins(game::game_plugin)
        .run();
}

#[cfg(debug_assertions)]
fn setup_logging() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or("error,phantom_of_scopa=debug".into());

    tracing_subscriber::Registry::default()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true),
        )
        .init();
}

#[cfg(not(debug_assertions))]
fn setup_logging() {
    use tracing_appender::rolling;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let file_appender = rolling::daily("logs", "phantom-of-scopa");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or("error,phantom_of_scopa=warn".into());

    tracing_subscriber::Registry::default()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    std::mem::forget(_guard);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_resources(app: &mut App) {
    let asset_server = app.world.resource::<AssetServer>();
    app.insert_resource(DefaultFont {
        font: asset_server.load(DEFAULT_FONT),
    });
    let asset_server = app.world.resource::<AssetServer>();
    app.insert_resource(OneShotAudio {
        card_draw: asset_server.load("audio/Card_Deal02.ogg"),
        card_put: asset_server.load("audio/Card_place02.ogg"),
    });
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

pub fn spawn_audio(
    mut commands: Commands,
    mut audio_events: EventReader<events::PlayAudio>,
    one_shot: Res<OneShotAudio>,
    config: Res<Config>,
) {
    for event in audio_events.read() {
        #[allow(unreachable_patterns)]
        let asset = match event {
            events::PlayAudio::DrawHand => one_shot.card_draw.clone(),
            events::PlayAudio::PutCard => one_shot.card_put.clone(),
        };
        let volume = config.volume_as_f32();
        commands.spawn(AudioBundle {
            source: asset,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(volume),
                spatial: false,
                paused: false,
                ..default()
            },
        });
    }
}
