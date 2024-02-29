use bevy::audio::Volume;
use bevy::prelude::*;

use super::{despawn_screen, AppState};
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
pub const HAND_SLOT_WIDTH: f32 = 78.0;
pub const HAND_SLOT_HEIGHT: f32 = 113.0;
pub const DEFAULT_VOLUME: f32 = 0.1;

#[derive(Component)]
struct InGameComponent;

#[derive(Component)]
struct PlayerHandArea;

#[derive(Component)]
struct PlayerHandSlot;

#[derive(Component)]
struct PlayerCard(UiCard);

#[derive(Component)]
struct SelectedCard;

#[derive(Component)]
struct CardImage;

#[derive(Component)]
struct GameButton;

#[derive(Component)]
struct TakeButton;

#[derive(Component)]
struct PutButton;

#[derive(Component)]
struct HighlightImage;

#[derive(Component)]
struct SoundEffect;

#[derive(Event)]
enum GameEvent {
    NewHand(Vec<Card>),
}

#[derive(Resource)]
struct HandSlots {
    pub entities: Vec<Entity>,
}

impl HandSlots {
    fn new() -> Self {
        Self {
            entities: Vec::with_capacity(3),
        }
    }

    fn add(&mut self, card_slot: Entity) {
        self.entities.push(card_slot);
    }
}

#[derive(Debug, Clone, Copy)]
struct UiCard {
    card: Card,
}

impl UiCard {
    fn new(card: Card) -> Self {
        Self { card }
    }

    fn asset_path(&self) -> String {
        use Suite::*;
        let suite = match self.card.suite {
            Clubs => "clubs",
            Coins => "coins",
            Cups => "cups",
            Swords => "swords",
        };
        let value = self.card.value().to_string();
        format!("cards/{}_{}.png", suite, value)
    }
}

#[derive(Resource)]
struct PlayerHand {
    pub hand: [Option<UiCard>; 3],
}

impl PlayerHand {
    fn new(hand: [UiCard; 3]) -> Self {
        Self {
            hand: [Some(hand[0]), Some(hand[1]), Some(hand[2])],
        }
    }

    fn put_on_table(&mut self, idx: usize) -> Option<UiCard> {
        self.hand[idx].take()
    }
}

pub fn game_plugin(app: &mut App) {
    let hand = PlayerHand::new([
        UiCard::new(Card::new(Suite::Swords, CardValue::Cavallo)),
        UiCard::new(Card::new(Suite::Coins, CardValue::Fante)),
        UiCard::new(Card::new(Suite::Cups, CardValue::Seven)),
    ]);
    // Add systems to app
    app.add_event::<GameEvent>()
        .add_systems(OnEnter(AppState::InGame), game_setup)
        .add_systems(Update, button_highlights)
        .add_systems(Update, put_button_pressed)
        .add_systems(Update, update_hand)
        .add_systems(Update, select_player_card)
        .add_systems(Update, update_selected_cards.after(select_player_card))
        .add_systems(OnExit(AppState::InGame), despawn_screen::<InGameComponent>)
        .insert_resource(hand);
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let mut hand_slots = HandSlots::new();
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
            hand_slots.add(create_player_hand_slot(parent, Val::Px(0.0)));
            hand_slots.add(create_player_hand_slot(
                parent,
                Val::Px(HAND_SLOT_WIDTH + HAND_CARDS_SPACING),
            ));
            hand_slots.add(create_player_hand_slot(
                parent,
                Val::Px(HAND_SLOT_WIDTH * 2.0 + HAND_CARDS_SPACING * 2.0),
            ));
        });
    commands.insert_resource(hand_slots);

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
}

fn create_player_hand_slot(parent: &mut ChildBuilder<'_>, left: Val) -> Entity {
    let hand_slot = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left,
                width: Val::Px(HAND_SLOT_WIDTH),
                height: Val::Px(HAND_SLOT_HEIGHT),
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

fn put_button_pressed(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PutButton>)>,
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
                println!("New card selected");
                commands.entity(id).insert(SelectedCard);
            } else {
                println!("Same card selected");
            }
            // In any case - remove "selected" marker from previously selected card. If it's the
            // same card as the currently selected one, then we deselect it.
            commands.entity(prev_selected_id).remove::<SelectedCard>();
        } else {
            commands.entity(id).insert(SelectedCard);
        }
    }
}

fn update_selected_cards(
    mut selected_cards_query: Query<Entity, (Added<SelectedCard>, With<CardImage>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    todo!()
}
