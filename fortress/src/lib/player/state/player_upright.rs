use control::{
    Controller,
    events::ControlEvent::{
        PlayerJump,
        PlayerMove,
        PlayerSlash,
    },
};
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use player::{
    PlayerState,
    state::{
        PlayerStateMachine,
        PlayerJumping,
    },
};

pub struct PlayerUpright;

impl PlayerStateMachine for PlayerUpright {
    fn pre_update(&mut self, player_state: &mut PlayerState, controller: &Controller, _dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>> {
        let move_dir = if controller.is_pressed(PlayerMove(LrDirection::Left)) {
            Some(LrDirection::Left)
        } else if controller.is_pressed(PlayerMove(LrDirection::Right)) {
            Some(LrDirection::Right)
        } else {
            None
        };
        player_state.body.move_horizontal(player_state.config.move_speed, move_dir);

        if controller.is_pressed(PlayerSlash) {
            player_state.try_slash();
        }

        if controller.just_pressed(PlayerJump) {
            return Some(Box::new(PlayerJumping::new(player_state)));
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
