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
    fn pre_update(&mut self, player_state: &mut PlayerState, _audio: &AudioPlayer, controller_id: ControllerId, controller: &Controller, _dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>> {
        player_state.set_velocity(PlayerState::compute_move_direction(controller_id, controller));

        None
    }
}

impl PlayerUpright {
    pub fn new() -> PlayerUpright {
        PlayerUpright {
        }
    }
}
