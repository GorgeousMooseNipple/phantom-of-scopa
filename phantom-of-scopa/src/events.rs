use bevy::ecs::event::Event;
use scopa_lib::card;

#[derive(Event)]
pub struct DrawEvent {
    pub hand: Vec<card::Card>,
}

#[derive(Event)]
pub enum PlayAudio {
    DrawHand,
}
