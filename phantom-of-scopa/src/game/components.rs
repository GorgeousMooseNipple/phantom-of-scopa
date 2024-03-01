use bevy::ecs::component::Component;
use bevy::ui::AlignSelf;
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PopUpLocation {
    Top,
    #[default]
    Bottom,
}

#[derive(Component, Debug)]
pub struct PopUpMessage {
    pub expiration_time: f64,
    pub location: PopUpLocation,
}

#[derive(Component, Debug)]
pub struct PopUpText;

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
