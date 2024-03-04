use super::components::*;
use super::InGameMenuState;
use crate::game::popups::PopUpEvent;
use crate::styles::*;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, mut next_state: ResMut<NextState<InGameMenuState>>) {
    commands.spawn((
        InGameMenuUI,
        InGameMenuRootNode,
        NodeBundle {
            style: Style {
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
            background_color: DEFAULT_BG.with_a(0.95).into(),
            ..default()
        },
    ));
    next_state.set(InGameMenuState::Root);
}

pub fn create_root_in_game_menu(
    root_q: Query<Entity, With<InGameMenuRootNode>>,
    mut popup_events: EventWriter<PopUpEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(root) = root_q.get_single() {
        commands.entity(root).with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                text: default_text("PHANTOM OF SCOPA", &asset_server),
                ..default()
            });
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
                        TextBundle {
                            text: default_text("Settings", &asset_server),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((InGameMenuUI, RootInGameMenuUI, default_button()))
                .with_children(|button| {
                    button.spawn((
                        InGameMenuUI,
                        RootInGameMenuUI,
                        ExitButton,
                        TextBundle {
                            text: default_text("Exit", &asset_server),
                            ..default()
                        },
                    ));
                });
        });
    } else {
        popup_events.send(PopUpEvent {
            text: "Can't find in-game menu root node".into(),
            ..default()
        });
    }
}

pub fn despawn_submenu(root_q: Query<Entity, With<InGameMenuRootNode>>, mut commands: Commands) {
    if let Ok(root) = root_q.get_single() {
        commands.entity(root).despawn_descendants();
    }
}

pub fn highlight_buttons(
    mut buttons_q: Query<
        (&mut BorderColor, &Interaction),
        (Changed<Interaction>, (With<Button>, With<InGameMenuUI>)),
    >,
) {
    for (mut border_color, interaction) in &mut buttons_q {
        match *interaction {
            Interaction::Hovered => *border_color = HOVERED_UI.into(),
            Interaction::Pressed => *border_color = SELECTED_UI.into(),
            Interaction::None => *border_color = INACTIVE_UI.into(),
        }
    }
}

pub fn open_settings(
    settings_button_q: Query<
        &Interaction,
        (Changed<Interaction>, (With<Button>, With<SettingsButton>)),
    >,
    mut next_state: ResMut<NextState<InGameMenuState>>,
) {
    if let Ok(Interaction::Pressed) = settings_button_q.get_single() {
        next_state.set(InGameMenuState::Settings);
    }
}

pub fn create_settings_in_game_menu(
    root_q: Query<Entity, With<InGameMenuRootNode>>,
    mut popup_events: EventWriter<PopUpEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let cur_volume = 5;
    if let Ok(root) = root_q.get_single() {
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                InGameMenuUI,
                SettingsUi,
                TextBundle {
                    style: Style {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    text: default_text("Volume", &asset_server),
                    ..default()
                },
            ));
            parent
                .spawn((
                    SettingsUi,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Percent(10.0),
                            margin: UiRect::bottom(Val::Px(20.0)),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceEvenly,
                            ..default()
                        },
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
                        TextBundle {
                            text: default_text("Back", &asset_server),
                            ..default()
                        },
                    ));
                });
        });
    } else {
        popup_events.send(PopUpEvent {
            text: "Can't find in-game menu root node".into(),
            ..default()
        });
    }
}

fn create_volume_button(
    parent: &mut ChildBuilder<'_>,
    volume_level: usize,
    selected: bool,
) -> Entity {
    let mut commands = parent.spawn((
        InGameMenuUI,
        SettingsUi,
        VolumeSettingsButton(volume_level),
        ButtonBundle {
            style: Style {
                width: Val::Px(16.0),
                ..default()
            },
            background_color: INACTIVE_UI.into(),
            ..default()
        },
    ));
    if selected {
        commands.insert(SelectedVolume);
    }
    commands.id()
}

pub fn highlight_volume_buttons(
    mut volume_buttons_q: Query<
        (&Interaction, Entity, &mut BackgroundColor),
        (
            Changed<Interaction>,
            (With<VolumeSettingsButton>, Without<SelectedVolume>),
        ),
    >,
    mut selected_q: Query<(Entity, &mut BackgroundColor), With<SelectedVolume>>,
    mut commands: Commands,
) {
    for (interaction, id, mut bg_color) in &mut volume_buttons_q {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = SELECTED_UI.into();
                if let Ok((prev_selected, mut prev_bg)) = selected_q.get_single_mut() {
                    commands.entity(prev_selected).remove::<SelectedVolume>();
                    *prev_bg = INACTIVE_UI.into();
                }
                commands.entity(id).insert(SelectedVolume);
            }
            Interaction::Hovered => *bg_color = HOVERED_INACTIVE_UI.into(),
            Interaction::None => *bg_color = INACTIVE_UI.into(),
        }
    }
}

pub fn selected_volume_button(mut selected_q: Query<&mut BackgroundColor, With<SelectedVolume>>) {
    if let Ok(mut selected_bg) = selected_q.get_single_mut() {
        *selected_bg = SELECTED_UI.into();
    }
}

pub fn back_to_root(
    settings_button_q: Query<
        &Interaction,
        (Changed<Interaction>, (With<Button>, With<BackToRootButton>)),
    >,
    mut next_state: ResMut<NextState<InGameMenuState>>,
) {
    if let Ok(Interaction::Pressed) = settings_button_q.get_single() {
        next_state.set(InGameMenuState::Root);
    }
}

pub fn close_menu(
    ingame_menu_ui_q: Query<Entity, With<InGameMenuRootNode>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<InGameMenuState>>,
) {
    for ui in &ingame_menu_ui_q {
        commands.entity(ui).despawn_recursive();
    }
    next_state.set(InGameMenuState::Closed);
}
