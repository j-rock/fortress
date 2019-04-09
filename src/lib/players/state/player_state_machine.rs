use crate::{
    audio::AudioPlayer,
    control::{
        Controller,
        ControllerId,
    },
    dimensions::time::DeltaTime,
    players::PlayerState
};

pub trait PlayerStateMachine {
    // Before physics step.
    fn pre_update(&mut self, player_state: &mut PlayerState, audio: &AudioPlayer, controller_id: ControllerId, controller: &Controller, dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>>;

    // After physics step.
    fn post_update(&mut self, _player_state: &PlayerState, _audio: &AudioPlayer) -> Option<Box<dyn PlayerStateMachine>> {
        None
    }
}