use crate::{
    audio::AudioPlayer,
    control::{
        Controller,
        ControllerId,
    },
    dimensions::time::DeltaTime,
    players::{
        PlayerState,
        state::PlayerStateMachine,
    }
};

#[derive(Default)]
pub struct PlayerUpright;

impl PlayerStateMachine for PlayerUpright {
    fn pre_update(&mut self, _player_state: &mut PlayerState, _audio: &AudioPlayer, _controller_id: ControllerId, _controller: &Controller, _dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>> {
        None
    }
}

impl PlayerUpright {
    pub fn new() -> PlayerUpright {
        PlayerUpright {
        }
    }
}
