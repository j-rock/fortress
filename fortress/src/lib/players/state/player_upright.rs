use audio::AudioPlayer;
use control::{
    Controller,
    ControllerId,
    events::ControlEvent::{
        PlayerFire,
        PlayerJump,
        PlayerMove,
        PlayerSlash,
    },
};
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use players::{
    PlayerState,
    state::{
        PlayerStateMachine,
        PlayerJumping,
    },
};

pub struct PlayerUpright;

impl PlayerStateMachine for PlayerUpright {
    fn pre_update(&mut self, player_state: &mut PlayerState, audio: &AudioPlayer, controller_id: ControllerId, controller: &Controller, _dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>> {
        let move_dir = if controller.is_pressed(controller_id, PlayerMove(LrDirection::Left)) {
            Some(LrDirection::Left)
        } else if controller.is_pressed(controller_id, PlayerMove(LrDirection::Right)) {
            Some(LrDirection::Right)
        } else {
            None
        };
        player_state.body.move_horizontal(player_state.stats.get_move_speed(), move_dir);

        if controller.is_pressed(controller_id, PlayerFire) {
            player_state.try_fire(audio);
        } else if controller.just_pressed(controller_id, PlayerSlash) {
            player_state.try_slash(audio);
        }

        if controller.just_pressed(controller_id, PlayerJump) {
            return Some(Box::new(PlayerJumping::new(player_state, audio)));
        }

        None
    }
}

impl PlayerUpright {
    pub fn new() -> PlayerUpright {
        PlayerUpright {
        }
    }
}
