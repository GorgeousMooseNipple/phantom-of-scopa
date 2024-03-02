use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use scopa_lib::card::*;

#[derive(Component, Debug)]
pub struct InGameComponent;

#[derive(Component, Debug)]
pub struct PlayerHandArea;

#[derive(Component, Debug)]
pub struct PlayerHandSlot;

#[derive(Component, Debug)]
pub struct PlayerCard(pub UiCard);

#[derive(Component, Debug)]
pub struct SelectedCard;

#[derive(Component, Debug)]
pub struct RemovedCardSelection;

#[derive(Component, Debug)]
pub struct CardImage;

#[derive(Component, Debug)]
pub struct GameButton;

#[derive(Component, Debug)]
pub struct TakeButton;

#[derive(Component, Debug)]
pub struct PutButton;

#[derive(Component, Debug)]
pub struct HighlightImage;

#[derive(Component, Debug)]
pub struct SoundEffect;

#[derive(Component, Debug)]
pub struct TableArea;

#[derive(Component, Debug)]
pub struct TableSlot;

#[derive(Component, Debug)]
pub struct TableCard;

#[derive(Component, Debug)]
pub struct Draggable;

#[derive(Component, Debug)]
pub struct Dragged {
    previous_parent: Entity,
}

impl Dragged {
    pub fn leaving(parent: Entity) -> Self {
        Self {
            previous_parent: parent,
        }
    }

    pub fn return_to(&self) -> Entity {
        self.previous_parent
    }
}

#[derive(Component, Debug)]
pub struct DropIn;

#[derive(Component, Debug)]
pub struct CursorMarker;

#[derive(Debug, Clone, Copy)]
pub struct UiCard {
    card: Card,
}

impl UiCard {
    pub fn new(card: Card) -> Self {
        Self { card }
    }

    pub fn asset_path(&self) -> String {
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
