#![allow(clippy::type_complexity)]
mod components;
mod popups;

use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use super::{despawn_screen, AppState};
use components::*;
use popups::*;
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
pub const TABLE_WIDTH: f32 = 410.0;
pub const TABLE_HEIGHT: f32 = 234.0;
pub const TABLE_X: f32 = 196.0;
pub const TABLE_Y: f32 = 124.0;
pub const TABLE_SLOT_WIDTH: f32 = 77.0;
pub const TABLE_SLOT_HEIGHT: f32 = 111.0;

#[derive(Event)]
enum GameEvent {
    NewHand(Vec<Card>),
}

#[derive(Resource)]
struct SelectedCardImage(Handle<Image>);

struct TableSlotEntity {
    id: Entity,
    vacant: bool,
}

impl TableSlotEntity {
    fn new(id: Entity) -> Self {
        Self { id, vacant: true }
    }

    fn id(&self) -> Entity {
        self.id
    }

    fn is_vacant(&self) -> bool {
        self.vacant
    }

    fn occupy(&mut self) {
        self.vacant = false;
    }
}

#[derive(Resource)]
struct TableSlots {
    pub slots: Vec<TableSlotEntity>,
}

impl TableSlots {
    fn new(slots: Vec<TableSlotEntity>) -> Self {
        Self { slots }
    }

    fn add(&mut self, entity: Entity) {
        self.slots.push(TableSlotEntity::new(entity));
    }

    fn insert(&mut self) -> Option<Entity> {
        for i in 0..self.slots.len() {
            if self.slots[i].is_vacant() {
                self.slots[i].occupy();
                return Some(self.slots[i].id());
            }
        }
        None
    }
}

#[derive(Resource)]
struct CursorEntity {
    entity: Entity,
    position: Vec2,
}

impl CursorEntity {
    fn new(entity: Entity, position: Vec2) -> Self {
        Self { entity, position }
    }

    fn entity(&self) -> Entity {
        self.entity
    }

    fn update_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    fn current_position(&self) -> Vec2 {
        self.position
    }
}

pub fn game_plugin(app: &mut App) {
    // Add systems to app
    app.add_event::<GameEvent>()
        .add_event::<PopUpEvent>()
        .add_systems(OnEnter(AppState::InGame), game_setup)
        .add_systems(Update, handle_popups)
        .add_systems(Update, clear_expired_popups)
        .add_systems(Update, button_highlights)
        .add_systems(Update, take_button_pressed)
        .add_systems(Update, update_hand)
        .add_systems(Update, handle_drag)
        .add_systems(Update, select_hand_card)
        .add_systems(Update, select_table_card)
        .add_systems(
            Update,
            update_selected_cards
                .after(select_hand_card)
                .after(select_table_card),
        )
        .add_systems(Update, update_cursor_entity)
        .add_systems(Update, put_button_pressed)
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
    let mut table_slots = TableSlots::new(Vec::with_capacity(10));
    commands
        .spawn((
            TableArea,
            NodeBundle {
                style: Style {
                    width: Val::Px(TABLE_WIDTH),
                    height: Val::Px(TABLE_HEIGHT),
                    position_type: PositionType::Absolute,
                    left: Val::Px(TABLE_X),
                    top: Val::Px(TABLE_Y),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TableArea,
                DropIn,
                HighlightImage,
                Interaction::None,
                ImageBundle {
                    style: Style {
                        width: Val::Px(TABLE_WIDTH),
                        height: Val::Px(TABLE_HEIGHT),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("table_highlight.png")),
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ));
            table_slots.add(create_table_slot(parent, 0, 1));
            table_slots.add(create_table_slot(parent, 1, 3));
            table_slots.add(create_table_slot(parent, 1, 1));
            table_slots.add(create_table_slot(parent, 0, 3));
            table_slots.add(create_table_slot(parent, 0, 2));
            table_slots.add(create_table_slot(parent, 1, 2));
            table_slots.add(create_table_slot(parent, 0, 0));
            table_slots.add(create_table_slot(parent, 1, 4));
            table_slots.add(create_table_slot(parent, 1, 0));
            table_slots.add(create_table_slot(parent, 0, 4));
        });
    commands.insert_resource(table_slots);

    // Cursor tracking entity for drag and drop
    let cursor_entity = CursorEntity::new(
        commands
            .spawn((
                CursorMarker,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(1.0),
                        height: Val::Px(1.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
            ))
            .id(),
        Vec2::from_array([0.0, 0.0]),
    );
    commands.insert_resource(cursor_entity);
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

fn create_table_slot(parent: &mut ChildBuilder<'_>, row: usize, column: usize) -> Entity {
    let x_offset = BORDER_WIDTH + (TABLE_SLOT_WIDTH + BORDER_WIDTH) * column as f32;
    let y_offset = BORDER_WIDTH + (TABLE_SLOT_HEIGHT + BORDER_WIDTH) * row as f32;
    parent
        .spawn((
            TableSlot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(CARD_SLOT_WIDTH),
                    height: Val::Px(CARD_SLOT_HEIGHT),
                    left: Val::Px(x_offset),
                    top: Val::Px(y_offset),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id()
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

fn put_button_pressed(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PutButton>)>,
    player_selected_card: Query<Entity, (With<PlayerCard>, With<SelectedCard>)>,
    table_selected_cards: Query<Entity, (With<TableCard>, With<SelectedCard>)>,
    asset_server: Res<AssetServer>,
    mut popup_events: EventWriter<PopUpEvent>,
    mut table_slots: ResMut<TableSlots>,
    mut commands: Commands,
) {
    if let Ok(card_id) = player_selected_card.get_single() {
        for interaction in &interaction_query {
            if let Interaction::Pressed = *interaction {
                if let Some(slot_id) = table_slots.insert() {
                    let mut card = commands.entity(card_id);
                    card.remove::<SelectedCard>();
                    card.insert(RemovedCardSelection);
                    card.remove::<PlayerCard>();
                    card.insert(TableCard);
                    card.remove::<Draggable>();
                    card.remove::<RelativeCursorPosition>();
                    card.set_parent(slot_id);
                    commands.spawn((
                        SoundEffect,
                        AudioBundle {
                            source: asset_server.load("audio/Card_place02.ogg"),
                            settings: PlaybackSettings {
                                mode: bevy::audio::PlaybackMode::Once,
                                volume: Volume::new(DEFAULT_VOLUME),
                                paused: false,
                                spatial: false,
                                ..default()
                            },
                        },
                    ));
                } else {
                    popup_events.send(PopUpEvent {
                        text: "The table is full".into(),
                        duration: 2.0,
                        location: PopUpLocation::Bottom,
                    });
                }
                // Remove selection from all selected cards on the table
                for selected_id in &table_selected_cards {
                    let mut selected_card = commands.entity(selected_id);
                    selected_card.remove::<SelectedCard>();
                    selected_card.insert(RemovedCardSelection);
                }
            }
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
                                Draggable,
                                RelativeCursorPosition::default(),
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

fn select_hand_card(
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

fn select_table_card(
    interacted_card_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<TableCard>)>,
    selected_card_query: Query<Entity, (With<TableCard>, With<SelectedCard>)>,
    mut commands: Commands,
) {
    if let Ok((id, Interaction::Pressed)) = interacted_card_query.get_single() {
        let mut card = commands.entity(id);
        // If this card is already selected, then remove it's selection. Select it otherwise.
        if selected_card_query.get(id).is_ok() {
            card.remove::<SelectedCard>();
            card.insert(RemovedCardSelection);
        } else {
            card.insert(SelectedCard);
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

fn handle_drag(
    mut draggable_query: Query<
        (Entity, &Parent, &RelativeCursorPosition),
        (With<Draggable>, Without<Dragged>),
    >,
    dragged_query: Query<(Entity, &Dragged), With<Dragged>>,
    mouse_pressed: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    cursor_entity: Res<CursorEntity>,
) {
    if mouse_pressed.just_pressed(MouseButton::Left) {
        if let Some((id, parent, _)) = draggable_query.iter_mut().find(|(_, _, c)| c.mouse_over()) {
            println!("Just pressed on draggable!");
            commands
                .entity(id)
                .insert(Dragged::leaving(parent.get()))
                .remove_parent_in_place()
                .set_parent(cursor_entity.entity());
        }
    }
    if mouse_pressed.just_released(MouseButton::Left) {
        if let Ok((id, dragged)) = dragged_query.get_single() {
            println!("Released dragged!");
            commands
                .entity(id)
                .set_parent(dragged.return_to())
                .remove::<Dragged>();
        }
    }
}

fn update_cursor_entity(
    mut cursor_query: Query<&mut Style, With<CursorMarker>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for moved in cursor_moved_events.read() {
        if let Ok(mut cursor_entity_style) = cursor_query.get_single_mut() {
            cursor_entity_style.left = Val::Px(moved.position.x);
            cursor_entity_style.top = Val::Px(moved.position.y);
        }
    }
}

fn highlight_on_drag(
    drop_area_query: Query<(&Interaction, &mut Visibility), (With<DropIn>, With<HighlightImage>)>,
) {
    todo!()
}
