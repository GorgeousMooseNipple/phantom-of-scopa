#![allow(unused)]
use bevy::prelude::*;
use scopa_lib::card::*;

#[derive(Component, Debug)]
pub struct InGameComponent;

#[derive(Component, Debug)]
pub struct SizedArea {
    pub size: Vec2,
}

#[derive(Bundle, Debug)]
pub struct LogicalArea {
    pub area: SizedArea,
    pub sprite: Sprite,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl LogicalArea {
    pub fn with_size(size: Vec2) -> Self {
        Self {
            area: SizedArea { size: size },
            sprite: Sprite {
                color: Color::default().with_a(0.),
                custom_size: Some(size),
                ..default()
            },
            global_transform: GlobalTransform::default(),
            texture: Handle::<Image>::Weak(AssetId::default()),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

#[derive(Component, Debug)]
pub struct DebugSprite {
    pub alpha: f32,
    pub color: Color,
}

impl DebugSprite {
    pub fn with_color(color: Color) -> Self {
        Self { alpha: 0.5, color }
    }
}

impl Default for DebugSprite {
    fn default() -> Self {
        Self {
            alpha: 0.5,
            color: Color::rgba(1., 0., 0., 0.3),
        }
    }
}

#[derive(Component, Debug)]
pub struct WithOverlay {
    pub texture: Handle<Image>,
    pub overlay: Option<Entity>,
    pub on_drag: bool,
}

impl WithOverlay {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            texture,
            overlay: None,
            on_drag: false,
        }
    }

    pub fn only_on_drag(mut self) -> Self {
        self.on_drag = true;
        return self;
    }
}

#[derive(Component, Debug)]
pub struct Table;

#[derive(Component, Debug)]
pub struct TablePlayableArea;

#[derive(Component, Debug)]
pub struct OverlayOnDrag;

#[derive(Component, Debug)]
pub struct HighlightOverlay;

#[derive(Component, Debug)]
pub struct PlayerHandArea;

#[derive(Component, Debug)]
pub struct OpponentHandArea;

#[derive(Component, Debug)]
pub struct PlayerCardSlot;

#[derive(Component, Debug)]
pub struct OpponentCardSlot;

#[derive(Component, Debug)]
pub struct TableCardSlot;

#[derive(Component, Debug)]
pub struct TableSlotsOrder {
    pub slots: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct OccupiedSlot;

#[derive(Component, Debug)]
pub struct PlayerCard {
    pub card: Card,
}

#[derive(Component, Debug)]
pub struct OpponentCard;

#[derive(Component, Debug)]
pub struct TableCard {
    pub card: Card,
}

#[derive(Component, Debug)]
pub struct AtSlot {
    pub slot: Entity,
}

#[derive(Component, Debug)]
pub struct PlayerTakenPile;

#[derive(Component, Debug)]
pub struct OpponentTakenPile;

#[derive(Component, Debug)]
pub struct PlayerName;

#[derive(Component, Debug)]
pub struct OpponentName;

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
