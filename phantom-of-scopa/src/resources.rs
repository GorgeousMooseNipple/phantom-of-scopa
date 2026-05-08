use bevy::asset::Handle;
use bevy::audio::AudioSource;
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct OneShotAudio {
    pub card_draw: Handle<AudioSource>,
    pub card_put: Handle<AudioSource>,
}
