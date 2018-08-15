use control::{
    Controller,
    events::ControlEvent::{
        PlayerJump,
        PlayerMove,
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
        SlashState,
    },
};

pub struct PlayerUpright {
    slash_state: SlashState,
}

impl PlayerStateMachine for PlayerUpright {
    fn pre_update(&mut self, player_state: &mut PlayerState, controller: &Controller, dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>> {
        let move_dir = if controller.is_pressed(PlayerMove(LrDirection::Left)) {
            Some(LrDirection::Left)
        } else if controller.is_pressed(PlayerMove(LrDirection::Right)) {
            Some(LrDirection::Right)
        } else {
            None
        };
        player_state.body.move_horizontal(player_state.config.move_speed, move_dir);

        self.slash_state.update(player_state, controller, dt);

        if controller.just_pressed(PlayerJump) {
            return Some(Box::new(PlayerJumping::new(player_state, self.slash_state)));
        }

        None
    }
}

impl PlayerUpright {
    pub fn new(slash_state: SlashState) -> PlayerUpright {
        PlayerUpright {
            slash_state
        }
    }
}
