mod components;

use bevy::audio::Volume;
use bevy::prelude::*;

use super::{despawn_screen, AppState};
use components::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use scopa_lib::card::*;

pub const HAND_WIDTH: f32 = 282.0;
pub const HAND_HEIGHT: f32 = 113.0;
pub const CARD_WIDTH: f32 = 71.0;
pub const CARD_HEIGHT: f32 = 110.0;
pub const HAND_CARDS_SPACING: f32 = 24.0;
pub const PLAYER_HAND_X: f32 = 260.0;
pub const PLAYER_HAND_Y: f32 = 366.0;
pub const CARD_SLOT_WIDTH: f32 = 78.0;
pub const CARD_SLOT_HEIGHT: f32 = 113.0;
pub const DEFAULT_VOLUME: f32 = 0.1;
pub const BORDER_WIDTH: f32 = 4.0;

#[derive(Event)]
enum GameEvent {
    NewHand(Vec<Card>),
}

#[derive(Resource)]
struct SelectedCardImage(Handle<Image>);

struct TableSlot {
    id: Entity,
    occupied: bool,
}

pub fn game_plugin(app: &mut App) {
    // Add systems to app
    app.add_event::<GameEvent>()
        .add_systems(OnEnter(AppState::InGame), game_setup)
        .add_systems(Update, button_highlights)
        .add_systems(Update, take_button_pressed)
        .add_systems(Update, update_hand)
        .add_systems(Update, select_player_card)
        .add_systems(Update, update_selected_cards.after(select_player_card))
        .add_systems(OnExit(AppState::InGame), despawn_screen::<InGameComponent>);
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Insert image of selected card as a resource
    commands.insert_resource(SelectedCardImage(asset_server.load("card_selected.png")));
    // Spawn table background image
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            InGameComponent,
        ))
        .with_children(|parent| {
            let table = asset_server.load("table.png");
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                image: UiImage::new(table),
                ..default()
            });
        });

    // Spawn player's hand
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PLAYER_HAND_X),
                    top: Val::Px(PLAYER_HAND_Y),
                    width: Val::Px(HAND_WIDTH),
                    height: Val::Px(HAND_HEIGHT),
                    ..default()
                },
                ..default()
            },
            InGameComponent,
            PlayerHandArea,
        ))
        .with_children(|parent| {
            create_player_hand_slot(parent, Val::Px(0.0));
            create_player_hand_slot(parent, Val::Px(CARD_SLOT_WIDTH + HAND_CARDS_SPACING));
            create_player_hand_slot(
                parent,
                Val::Px(CARD_SLOT_WIDTH * 2.0 + HAND_CARDS_SPACING * 2.0),
            );
        });

    // Create buttons
    // Take button
    let take_button_image = asset_server.load("take_button.png");
    let take_button_highlight_image = asset_server.load("take_button_highlight.png");
    let take_button = ButtonBundle {
        image: UiImage::new(take_button_image),
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(669.0),
            top: Val::Px(187.0),
            ..default()
        },
        ..default()
    };
    commands
        .spawn((take_button, GameButton, TakeButton))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(take_button_highlight_image),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                HighlightImage,
            ));
        });

    // Put button
    let put_button_image = asset_server.load("put_button.png");
    let put_button_highlight_image = asset_server.load("put_button_highlight.png");
    let put_button = ButtonBundle {
        image: UiImage::new(put_button_image),
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(669.0),
            top: Val::Px(242.0),
            ..default()
        },
        ..default()
    };
    commands
        .spawn((put_button, GameButton, PutButton))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(put_button_highlight_image),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                HighlightImage,
            ));
        });

    // Spawn table
    todo!()
}

fn create_player_hand_slot(parent: &mut ChildBuilder<'_>, left: Val) -> Entity {
    let hand_slot = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left,
                width: Val::Px(CARD_SLOT_WIDTH),
                height: Val::Px(CARD_SLOT_HEIGHT),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        PlayerHandSlot,
    );
    parent.spawn(hand_slot).id()
}

fn button_highlights(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<GameButton>)>,
    mut image_query: Query<&mut Visibility, With<HighlightImage>>,
) {
    for (interaction, children) in &interaction_query {
        let mut highlight_visibility = image_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Hovered => {
                if *highlight_visibility != Visibility::Visible {
                    *highlight_visibility = Visibility::Visible;
                }
            }
            Interaction::None => {
                if *highlight_visibility != Visibility::Hidden {
                    *highlight_visibility = Visibility::Hidden;
                }
            }
            _ => {}
        }
    }
}

fn random_hand() -> Vec<Card> {
    let mut rand_hand: Vec<Card> = Vec::with_capacity(3);
    let mut rng = thread_rng();
    for _ in 0..3 {
        let suite = *[Suite::Coins, Suite::Clubs, Suite::Cups, Suite::Swords]
            .choose(&mut rng)
            .unwrap();
        let value = *[
            CardValue::One,
            CardValue::Two,
            CardValue::Three,
            CardValue::Four,
            CardValue::Five,
            CardValue::Six,
            CardValue::Seven,
            CardValue::Fante,
            CardValue::Cavallo,
            CardValue::Re,
        ]
        .choose(&mut rng)
        .unwrap();
        rand_hand.push(Card::new(suite, value));
    }
    rand_hand
}

fn take_button_pressed(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TakeButton>)>,
    mut game_events: EventWriter<GameEvent>,
) {
    for interaction in &interaction_query {
        if let Interaction::Pressed = *interaction {
            game_events.send(GameEvent::NewHand(random_hand()));
        }
    }
}

fn update_hand(
    mut game_events: EventReader<GameEvent>,
    mut hand_slots_query: Query<(Entity, Option<&Children>), With<PlayerHandSlot>>,
    mut slot_image_query: Query<&mut UiImage, With<PlayerCard>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in game_events.read() {
        #[allow(clippy::single_match)]
        match event {
            GameEvent::NewHand(hand) => {
                let cards = hand
                    .iter()
                    .map(|card| UiCard::new(*card))
                    .collect::<Vec<UiCard>>();
                for ((slot, slot_children), card) in hand_slots_query.iter_mut().zip(cards) {
                    let new_card_image = UiImage::new(asset_server.load(card.asset_path()));
                    if let Some(children) = slot_children {
                        let slot_image = children.first().unwrap();
                        let mut card_image = slot_image_query.get_mut(*slot_image).unwrap();
                        *card_image = new_card_image;
                    } else {
                        let new_slot_image = commands
                            .spawn((
                                PlayerCard(card),
                                CardImage,
                                Interaction::None,
                                ImageBundle {
                                    style: Style {
                                        width: Val::Px(CARD_WIDTH),
                                        height: Val::Px(CARD_HEIGHT),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: new_card_image,
                                    ..default()
                                },
                            ))
                            .id();
                        commands.entity(slot).add_child(new_slot_image);
                    }
                    // Play hand deal sound
                    commands.spawn((
                        SoundEffect,
                        AudioBundle {
                            source: asset_server.load("audio/Card_Deal02.ogg"),
                            settings: PlaybackSettings {
                                mode: bevy::audio::PlaybackMode::Once,
                                volume: Volume::new(DEFAULT_VOLUME),
                                paused: false,
                                spatial: false,
                                ..default()
                            },
                        },
                    ));
                }
            }
            _ => {}
        }
    }
}

fn select_player_card(
    interacted_card_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<PlayerCard>)>,
    selected_card_query: Query<Entity, (With<PlayerCard>, With<SelectedCard>)>,
    mut commands: Commands,
) {
    if let Ok((id, Interaction::Pressed)) = interacted_card_query.get_single() {
        // If some card in player's hand have been already selected we need to remove it's
        // "SelectedCard" marker component as only one card in player's hand can be selected at a
        // time
        if let Ok(prev_selected_id) = selected_card_query.get_single() {
            // If currently selected card differs from previously selected one, mark it as
            // "selected" by adding "SelectedCard" component.
            if id != prev_selected_id {
                commands.entity(id).insert(SelectedCard);
            }
            // In any case - remove "selected" marker from previously selected card. If it's the
            // same card as the currently selected one, then we deselect it.
            let mut prev_selected = commands.entity(prev_selected_id);
            prev_selected.remove::<SelectedCard>();
            prev_selected.insert(RemovedCardSelection);
        } else {
            commands.entity(id).insert(SelectedCard);
        }
    }
}

fn create_selected_image(image: Handle<Image>) -> ImageBundle {
    ImageBundle {
        style: Style {
            width: Val::Px(CARD_WIDTH),
            height: Val::Px(CARD_HEIGHT),
            align_self: AlignSelf::Center,
            ..default()
        },
        image: UiImage::new(image),
        ..default()
    }
}

fn update_selected_cards(
    selected_cards_query: Query<Entity, (With<CardImage>, Added<SelectedCard>)>,
    deselected_cards_query: Query<Entity, (With<CardImage>, With<RemovedCardSelection>)>,
    mut commands: Commands,
    selected_image: Res<SelectedCardImage>,
) {
    for id in &selected_cards_query {
        let selected_image_id = commands
            .spawn(create_selected_image(selected_image.0.clone()))
            .id();
        commands.entity(id).add_child(selected_image_id);
    }
    for id in &deselected_cards_query {
        let mut deselected_card = commands.entity(id);
        deselected_card.remove::<RemovedCardSelection>();
        // Those entities can only have selection image as a child, so clear all children
        // Kind of unclear which of these two methods to use in this case
        // deselected_card.clear_children();
        deselected_card.despawn_descendants();
    }
}
