use bevy::ecs::component::Component;
use scopa_lib::card::*;

#[derive(Component)]
pub struct InGameComponent;

#[derive(Component)]
pub struct PlayerHandArea;

#[derive(Component)]
pub struct PlayerHandSlot;

#[derive(Component)]
pub struct PlayerCard(pub UiCard);

#[derive(Component)]
pub struct SelectedCard;

#[derive(Component)]
pub struct RemovedCardSelection;

#[derive(Component)]
pub struct CardImage;

#[derive(Component)]
pub struct GameButton;

#[derive(Component)]
pub struct TakeButton;

#[derive(Component)]
pub struct PutButton;

#[derive(Component)]
pub struct HighlightImage;

#[derive(Component)]
pub struct SoundEffect;

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
