use control::{
    Controller,
    ControlEvent::{
        PlayerJump,
        PlayerMove,
    }
};
use dimensions::{
    LrDirection,
    time::{
        self,
        DeltaTime,
    }
};
use liquidfun::box2d::common::math::Vec2;
use player::{
    PlayerState,
    state::{
        PlayerStateMachine,
        PlayerUpright,
        SlashState,
    }
};

pub struct PlayerJumping {
    has_hit_ground_again: bool,
    jumps_left: i32,
    current_delay: time::Microseconds,
    slash_state: SlashState,
}

impl PlayerStateMachine for PlayerJumping {
    fn pre_update(&mut self, player_state: &mut PlayerState, controller: &Controller, dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>> {
        self.current_delay -= dt.as_microseconds();
        if self.current_delay < 0 {
            self.current_delay = 0;
        }

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
            self.try_jump(player_state);
        }

        None
    }

    fn post_update(&mut self) -> Option<Box<dyn PlayerStateMachine>> {
        if self.has_hit_ground_again {
            Some(Box::new(PlayerUpright::new(self.slash_state)))
        } else {
            None
        }
    }

    fn make_foot_contact(&mut self) {
        self.has_hit_ground_again = true;
    }
}

impl PlayerJumping {
    pub fn new(player_state: &mut PlayerState, slash_state: SlashState) -> PlayerJumping {
        let mut jumping = PlayerJumping {
            has_hit_ground_again: false,
            jumps_left: player_state.config.num_jumps,
            current_delay: time::milliseconds(0),
            slash_state,
        };
        jumping.try_jump(player_state);
        jumping
    }

    pub fn try_jump(&mut self, player_state: &mut PlayerState) {
        if self.jumps_left > 0 && self.current_delay == 0 {
            self.current_delay = time::milliseconds(player_state.config.jump_delay_ms);
            self.jumps_left -= 1;

            let body = &player_state.body.body;
            let actual_body_velocity = *body.get_linear_velocity();
            let jump_boost = player_state.config.jump_strength - actual_body_velocity.y;
            let mass = body.get_mass();
            let impulse = Vec2::new(0.0, mass * jump_boost);
            let body_center = *body.get_world_center();
            body.apply_linear_impulse(&impulse, &body_center, true);
        }
    }
}
