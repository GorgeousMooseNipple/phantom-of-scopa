use super::components::*;
use crate::game::constants::*;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            InGameMenuUI,
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
                background_color: MENU_BG.with_a(0.95).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                text: Text::from_section(
                    "PHANTOM OF SCOPA",
                    TextStyle {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: DEFAULT_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                ),
                ..default()
            });
            create_menu_button("Settings", parent, &asset_server);
            create_menu_button("Exit", parent, &asset_server);
        });
}

fn create_menu_button(text: &str, parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            InGameMenuButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(BUTTON_WIDTH),
                    height: Val::Px(BUTTON_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: MENU_BG.into(),
                border_color: INACTIVE_UI.into(),
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: DEFAULT_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                ),
                ..default()
            });
        });
}

pub fn highlight_buttons(
    mut buttons_q: Query<
        (&mut BorderColor, &Interaction),
        (Changed<Interaction>, With<InGameMenuButton>),
    >,
) {
    for (mut border_color, interaction) in &mut buttons_q {
        match *interaction {
            Interaction::Hovered => *border_color = ACTIVE_UI.into(),
            Interaction::Pressed => *border_color = INTERACTED_UI.into(),
            Interaction::None => *border_color = INACTIVE_UI.into(),
        }
    }
}
