use super::components::*;
use super::InGameMenuState;
use crate::config::Config;
use crate::game::GameState;
use crate::popups::PopUpEvent;
use crate::styles::*;
use crate::AppState;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, mut next_state: ResMut<NextState<InGameMenuState>>) {
    commands.spawn((
        InGameMenuUI,
        InGameMenuRootNode,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(37.0),
            height: Val::Percent(60.0),
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(DEFAULT_BG.with_alpha(0.95)),
    ));
    next_state.set(InGameMenuState::Root);
}

pub fn create_root_in_game_menu(
    root_q: Query<Entity, With<InGameMenuRootNode>>,
    mut popup_events: MessageWriter<PopUpEvent>,
    mut commands: Commands,
    default_font: Res<DefaultFont>,
) {
    if let Ok(root) = root_q.single() {
        commands.entity(root).with_children(|parent| {
            parent.spawn(default_text("PHANTOM OF SCOPA", &default_font.font));
            parent
                .spawn((
                    InGameMenuUI,
                    RootInGameMenuUI,
                    SettingsButton,
                    default_button(),
                ))
                .with_children(|button| {
                    button.spawn((
                        InGameMenuUI,
                        RootInGameMenuUI,
                        default_text("Settings", &default_font.font),
                    ));
                });
            parent
                .spawn((
                    InGameMenuUI,
                    RootInGameMenuUI,
                    MainMenuButton,
                    default_button(),
                ))
                .with_children(|button| {
                    button.spawn((
                        InGameMenuUI,
                        RootInGameMenuUI,
                        default_text("Main menu", &default_font.font),
                    ));
                });
        });
    } else {
        popup_events.write(PopUpEvent {
            text: "Can't find in-game menu root node".into(),
            ..default()
        });
    }
}

pub fn despawn_submenu(root_q: Query<Entity, With<InGameMenuRootNode>>, mut commands: Commands) {
    if let Ok(root) = root_q.single() {
        commands.entity(root).despawn_children();
    }
}

pub fn open_settings(
    settings_button_q: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut next_state: ResMut<NextState<InGameMenuState>>,
) {
    if let Ok(Interaction::Pressed) = settings_button_q.single() {
        next_state.set(InGameMenuState::Settings);
    }
}

pub fn main_menu(
    main_menu_button_q: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Interaction::Pressed) = main_menu_button_q.single() {
        game_state.set(GameState::Playing);
        app_state.set(AppState::MainMenu);
    }
}

pub fn create_settings_in_game_menu(
    root_q: Query<Entity, With<InGameMenuRootNode>>,
    mut popup_events: MessageWriter<PopUpEvent>,
    mut commands: Commands,
    default_font: Res<DefaultFont>,
    config: Res<Config>,
) {
    let cur_volume = config.volume_level();
    if let Ok(root) = root_q.single() {
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                InGameMenuUI,
                SettingsUi,
                default_text("Volume", &default_font.font),
            ));
            parent
                .spawn((
                    SettingsUi,
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(10.0),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                ))
                .with_children(|volume_node| {
                    for level in 1..=10 {
                        if level == cur_volume {
                            create_volume_button(volume_node, level, true);
                        } else {
                            create_volume_button(volume_node, level, false);
                        }
                    }
                });
            parent
                .spawn((InGameMenuUI, SettingsUi, BackToRootButton, default_button()))
                .with_children(|button| {
                    button.spawn((
                        InGameMenuUI,
                        RootInGameMenuUI,
                        default_text("Back", &default_font.font),
                    ));
                });
        });
    } else {
        popup_events.write(PopUpEvent {
            text: "Can't find in-game menu root node".into(),
            ..default()
        });
    }
}

fn create_volume_button(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    volume_level: usize,
    selected: bool,
) -> Entity {
    let mut commands = parent.spawn((
        InGameMenuUI,
        SettingsUi,
        VolumeSettingsButton(volume_level),
        Button,
        Node {
            width: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(INACTIVE_UI),
    ));
    if selected {
        commands.insert(SelectedVolume);
    }
    commands.id()
}

pub fn highlight_volume_buttons(
    mut volume_buttons_q: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedVolume>),
        (Changed<Interaction>, With<VolumeSettingsButton>),
    >,
) {
    for (interaction, mut bg_color, selected) in &mut volume_buttons_q {
        match (*interaction, selected) {
            (Interaction::Hovered, Some(_)) => *bg_color = HOVERED_SELECTED_UI.into(),
            (Interaction::Hovered, None) => *bg_color = HOVERED_INACTIVE_UI.into(),
            (Interaction::None, Some(_)) => *bg_color = SELECTED_UI.into(),
            (Interaction::None, None) => *bg_color = INACTIVE_UI.into(),
            (Interaction::Pressed, _) => {}
        }
    }
}

pub fn selected_volume_button(
    interaction_q: Query<
        (&Interaction, Entity, &VolumeSettingsButton),
        (Changed<Interaction>, With<VolumeSettingsButton>),
    >,
    mut selected_q: Query<(Entity, &mut BackgroundColor), With<SelectedVolume>>,
    mut commands: Commands,
    mut config: ResMut<Config>,
    mut popup_events: MessageWriter<PopUpEvent>,
) {
    for (interaction, id, volume) in &interaction_q {
        if *interaction == Interaction::Pressed {
            if let Ok((prev_id, mut prev_bg)) = selected_q.single_mut() {
                *prev_bg = INACTIVE_UI.into();
                commands.entity(prev_id).remove::<SelectedVolume>();
            }
            commands.entity(id).insert(SelectedVolume);
            config.set_volume_level(volume.0);
            if let Err(e) = config.save() {
                popup_events.write(error_popup(e.to_string()));
            }
        }
    }
}

pub fn back_to_root(
    settings_button_q: Query<
        &Interaction,
        (Changed<Interaction>, (With<Button>, With<BackToRootButton>)),
    >,
    mut next_state: ResMut<NextState<InGameMenuState>>,
) {
    if let Ok(Interaction::Pressed) = settings_button_q.single() {
        next_state.set(InGameMenuState::Root);
    }
}

pub fn close_menu(
    ingame_menu_ui_q: Query<Entity, With<InGameMenuRootNode>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<InGameMenuState>>,
) {
    for ui in &ingame_menu_ui_q {
        commands.entity(ui).despawn();
    }
    next_state.set(InGameMenuState::Closed);
}
