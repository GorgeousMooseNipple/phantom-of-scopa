#![allow(unused)]
use super::components::*;
use super::resources::*;
use super::GameState;
use crate::config::Config;
use crate::error::{BaseError, Result};
use crate::events::*;
use crate::popups::*;
use crate::styles::*;
use bevy::render::texture::ImageLoaderSettings;
use bevy::render::texture::ImageSampler;
use scopa_lib::card;
use scopa_lib::event::GameEvent;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::lens::TransformScaleLens;
use bevy_tweening::{Animator, Delay, EaseFunction, Sequence, Tween};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Duration;

pub fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    default_font: Res<DefaultFont>,
    config: Res<Config>,
) {
    // Table background image
    let mut table = commands.spawn((InGameComponent, Table, Name::new("Main table sprite")));
    let table_bg: Handle<Image> = asset_server.load("table.png");
    table.insert(SpriteBundle {
        texture: table_bg,
        transform: Transform::from_xyz(-1., 0.5, TABLE_LAYER),
        ..default()
    });

    // Table logical area
    let playable_area_overlay = asset_server.load("table_highlight.png");
    let mut playable_area = commands.spawn((
        Name::new("Playable area on the table"),
        InGameComponent,
        TablePlayableArea,
        LogicalArea::with_size(Vec2::new(TABLE_WIDTH, TABLE_HEIGHT)),
        WithOverlay::new(playable_area_overlay),
        PickableBundle::default(),
        Transform::from_xyz(0., 0., AREA_LAYER),
    ));

    // Hand area
    let players_hand_area = commands
        .spawn((
            Name::new("Player's hand area"),
            InGameComponent,
            PlayerHandArea,
            LogicalArea::with_size(Vec2::new(HAND_WIDTH, HAND_HEIGHT)),
            Transform::from_xyz(PLAYER_HAND_X, PLAYER_HAND_Y, ON_TABLE_LAYER),
        ))
        .with_children(|hand| {
            add_hand_card_slot(hand, 0, PlayerCardSlot);
            add_hand_card_slot(hand, 1, PlayerCardSlot);
            add_hand_card_slot(hand, 2, PlayerCardSlot);
        });

    // Opponent's hand
    let opponents_hand_area = commands
        .spawn((
            Name::new("Opponent's hand area"),
            InGameComponent,
            OpponentHandArea,
            LogicalArea::with_size(Vec2::new(HAND_WIDTH, HAND_HEIGHT)),
            Transform::from_xyz(PLAYER_HAND_X, -PLAYER_HAND_Y, ON_TABLE_LAYER),
        ))
        .with_children(|hand| {
            add_hand_card_slot(hand, 0, OpponentCardSlot);
            add_hand_card_slot(hand, 1, OpponentCardSlot);
            add_hand_card_slot(hand, 2, OpponentCardSlot);
        });

    // Buttons
    let button_overlay: Handle<Image> = asset_server.load("button_highlight.png");
    // Take button
    let take_button_image = asset_server.load("take_button.png");
    let take_button = commands.spawn((
        Name::new("Take button"),
        InGameComponent,
        GameButton,
        TakeButton,
        SpriteBundle {
            texture: take_button_image,
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(TAKE_BUTTON_X, TAKE_BUTTON_Y, ON_TABLE_LAYER),
            ..default()
        },
        WithOverlay::new(button_overlay.clone()),
        PickableBundle::default(),
        On::<Pointer<Click>>::run(take_button_click),
    ));
    // Put button
    let put_button_image = asset_server.load("put_button.png");
    let put_button = commands.spawn((
        Name::new("Put button"),
        InGameComponent,
        GameButton,
        PutButton,
        SpriteBundle {
            texture: put_button_image,
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(PUT_BUTTON_X, PUT_BUTTON_Y, ON_TABLE_LAYER),
            ..default()
        },
        WithOverlay::new(button_overlay.clone()),
        PickableBundle::default(),
        On::<Pointer<Click>>::run(|| {
            println!("Put button clicked");
        }),
    ));

    // Player's taken pile
    let players_taken = commands.spawn((
        Name::new("Player's taken pile"),
        InGameComponent,
        PlayerTakenPile,
        DebugSprite::with_color(Color::PINK),
        LogicalArea::with_size(Vec2::new(CARD_W, CARD_H)),
        Transform::from_xyz(PLAYER_TAKEN_PILE_X, PLAYER_TAKEN_PILE_Y, ON_TABLE_LAYER)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(45.))),
    ));

    // Opponent's taken pile
    let opponents_taken = commands.spawn((
        Name::new("Opponent's taken pile"),
        InGameComponent,
        PlayerTakenPile,
        DebugSprite::with_color(Color::PURPLE),
        LogicalArea::with_size(Vec2::new(CARD_W, CARD_H)),
        Transform::from_xyz(OPPONENT_TAKEN_PILE_X, OPPONENT_TAKEN_PILE_Y, ON_TABLE_LAYER)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(45.))),
    ));

    // Player's name
    let players_name = commands
        .spawn((
            Name::new("Player's name"),
            InGameComponent,
            PlayerName,
            LogicalArea::with_size(Vec2::new(NAME_W, NAME_H)),
            Transform::from_xyz(PLAYER_NAME_X, PLAYER_NAME_Y, ON_TABLE_LAYER),
        ))
        .with_children(|name_slot| {
            name_slot.spawn(Text2dBundle {
                text: default_text(config.players_name(), &default_font.font),
                transform: Transform::from_xyz(0., 0., 1.0),
                ..default()
            });
        });

    // Opponent's name
    let opponents_name = commands.spawn((
        Name::new("Opponent's name"),
        InGameComponent,
        OpponentName,
        DebugSprite::with_color(Color::GREEN),
        LogicalArea::with_size(Vec2::new(NAME_W, NAME_H)),
        Transform::from_xyz(OPPONENT_NAME_X, OPPONENT_NAME_Y, ON_TABLE_LAYER),
    ));
}

fn add_hand_card_slot<S: Component>(mut hand: &mut ChildBuilder, index: usize, slot_component: S) {
    let slot_x =
        (CARD_SLOT_W + HAND_CARDS_SPACING) * index as f32 + (CARD_SLOT_W / 2.) - (HAND_WIDTH / 2.);
    let slot_y = 0.;
    hand.spawn((
        slot_component,
        Transform::from_xyz(slot_x, slot_y, 1.),
        GlobalTransform::default(),
        Visibility::Visible,
    ));
}

pub fn debug_areas(
    mut commands: Commands,
    areas: Query<(Entity, Option<&Name>, &Transform, &SizedArea, &DebugSprite), Added<DebugSprite>>,
) {
    if areas.is_empty() {
        return;
    }
    for (entity, name, transform, area, debug_sprite) in areas.iter() {
        commands.entity(entity).with_children(|parent| {
            let name_str = match name {
                Some(n) => n.as_str(),
                None => "Unnamed",
            };
            println!("Spawning debug sprite for '{}'", name_str);
            parent.spawn((
                Name::new(format!("Debug sprite for '{}'", name_str)),
                InGameComponent,
                SpriteBundle {
                    sprite: Sprite {
                        color: debug_sprite.color.with_a(debug_sprite.alpha),
                        custom_size: Some(area.size),
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., transform.translation.z + 2.),
                    visibility: Visibility::Visible,
                    ..default()
                },
                Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                },
            ));
        });
    }
}

pub fn attach_overlays(
    mut commands: Commands,
    mut highlightable: Query<
        (Entity, Option<&Name>, &Transform, &mut WithOverlay),
        Added<WithOverlay>,
    >,
) {
    if highlightable.is_empty() {
        return;
    }
    for (entity, name, transform, mut overlay) in highlightable.iter_mut() {
        let name_str = match name {
            Some(n) => n.as_str(),
            None => "Unnamed",
        };
        println!("Spawning highlight overlay for '{}'", name_str);
        let mut overlay_child = commands.spawn((
            Name::new(format!("Highlight overlay for '{}'", name_str)),
            InGameComponent,
            HighlightOverlay,
            SpriteBundle {
                texture: overlay.texture.clone(),
                transform: Transform::from_xyz(0., 0., transform.translation.z + 1.),
                visibility: Visibility::Hidden,
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ));
        overlay_child.set_parent(entity);
        overlay.overlay = Some(overlay_child.id());
    }
}

pub fn show_overlay_on_cursor_over(
    mut commands: Commands,
    mut cursor_over: EventReader<Pointer<Over>>,
    mut overlays: Query<(&Name, &mut Visibility), With<HighlightOverlay>>,
    highlightable: Query<(Entity, &WithOverlay), With<WithOverlay>>,
) {
    for over in cursor_over.read() {
        if let Ok((entity, overlay)) = highlightable.get(over.target) {
            if !overlay.overlay.is_some() || overlay.on_drag {
                continue;
            }
            if let Ok((name, mut visibility)) = overlays.get_mut(overlay.overlay.unwrap()) {
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn hide_overlay_on_cursor_out(
    mut cursor_out: EventReader<Pointer<Out>>,
    mut overlays: Query<(&Name, &mut Visibility), With<HighlightOverlay>>,
    highlightable: Query<(Entity, &WithOverlay), With<WithOverlay>>,
) {
    for out in cursor_out.read() {
        if let Ok((entity, overlay)) = highlightable.get(out.target) {
            if !overlay.overlay.is_some() || overlay.on_drag {
                continue;
            }
            if let Ok((name, mut visibility)) = overlays.get_mut(overlay.overlay.unwrap()) {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn toggle_in_game_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match cur_state.get() {
            GameState::Menu => next_state.set(GameState::Playing),
            GameState::Playing => next_state.set(GameState::Menu),
        }
    }
}

pub fn hide_overlays(mut overlays: Query<&mut Visibility, With<HighlightOverlay>>) {
    for mut visibility in overlays.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

pub fn take_button_click(
    take_btn_q: Query<Entity, With<TakeButton>>,
    mut draw_events: EventWriter<DrawEvent>,
) {
    let entity = take_btn_q.get_single().unwrap();
    let mut deck = card::Deck::default();
    deck.shuffle();
    let random_hand = deck.deal_hand();
    draw_events.send(DrawEvent {
        hand: Vec::from(random_hand),
    });
}

pub fn on_draw_hand(
    mut commands: Commands,
    mut events: EventReader<DrawEvent>,
    mut popups: EventWriter<PopUpEvent>,
    mut audio_events: EventWriter<PlayAudio>,
    slots: Query<Entity, (With<PlayerCardSlot>, Without<OccupiedSlot>)>,
    asset_server: Res<AssetServer>,
) {
    let available_slots: Vec<Entity> = slots.iter().collect();
    for event in events.read() {
        if event.hand.len() > available_slots.len() {
            popups.send(PopUpEvent {
                text: "Tried to draw hand, but available slots are missing".into(),
                ..default()
            });
            return;
        }
        for (i, card) in event.hand.iter().enumerate() {
            let &slot_entity = &available_slots[i];
            let ui_card = UiCard::new(card.clone());
            let card_image = asset_server.load_with_settings(
                ui_card.asset_path(),
                |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
            );
            let tween = Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(250),
                TransformScaleLens {
                    start: Vec3::splat(0.01),
                    end: Vec3::ONE,
                },
            );
            let sequence = Delay::new(Duration::from_millis(100 * i as u64 + 1)).then(tween);
            commands.entity(slot_entity).with_children(|slot| {
                slot.spawn((
                    Name::new(format!("Card: {}", &card)),
                    InGameComponent,
                    PlayerCard { card: *card },
                    SpriteBundle {
                        texture: card_image,
                        transform: Transform::from_xyz(0., 0., 1.0).with_scale(Vec3::splat(0.01)),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    Animator::new(sequence),
                ));
            });
            // .insert(OccupiedSlot);
        }
        audio_events.send(PlayAudio::DrawHand);
    }
}
