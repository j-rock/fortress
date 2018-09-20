use audio::{
    AudioPlayer,
    Sound,
};
use control::{
    Controller,
    ControllerId,
    ControlEvent::{
        PlayerFire,
        PlayerJump,
        PlayerMove,
        PlayerSlash,
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
    }
};

pub struct PlayerJumping {
    has_hit_ground_again: bool,
    jumps_left: i32,
    current_delay: time::Microseconds,
}

impl PlayerStateMachine for PlayerJumping {
    fn pre_update(&mut self, player_state: &mut PlayerState, audio: &AudioPlayer, controller_id: ControllerId, controller: &Controller, dt: DeltaTime) -> Option<Box<dyn PlayerStateMachine>> {
        self.current_delay -= dt.as_microseconds();
        if self.current_delay < 0 {
            self.current_delay = 0;
        }

        let move_dir = if controller.is_pressed(controller_id, PlayerMove(LrDirection::Left)) {
            Some(LrDirection::Left)
        } else if controller.is_pressed(controller_id, PlayerMove(LrDirection::Right)) {
            Some(LrDirection::Right)
        } else {
            None
        };
        player_state.body.move_horizontal(player_state.config.move_speed, move_dir);

        if controller.is_pressed(controller_id, PlayerFire) {
            player_state.try_fire(audio);
        } else if controller.just_pressed(controller_id, PlayerSlash) {
            player_state.try_slash(audio);
        }

        if controller.just_pressed(controller_id, PlayerJump) {
            self.try_jump(player_state, audio);
        }

        None
    }

    fn post_update(&mut self) -> Option<Box<dyn PlayerStateMachine>> {
        if self.has_hit_ground_again {
            Some(Box::new(PlayerUpright::new()))
        } else {
            None
        }
    }

    fn make_foot_contact(&mut self, audio: &AudioPlayer) {
        self.has_hit_ground_again = true;
        audio.play_sound(Sound::Plop);
    }
}

impl PlayerJumping {
    pub fn new(player_state: &mut PlayerState, audio: &AudioPlayer) -> PlayerJumping {
        let mut jumping = PlayerJumping {
            has_hit_ground_again: false,
            jumps_left: player_state.config.num_jumps,
            current_delay: time::milliseconds(0),
        };
        jumping.try_jump(player_state, audio);
        jumping
    }

    pub fn try_jump(&mut self, player_state: &mut PlayerState, audio: &AudioPlayer) {
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

            audio.play_sound(Sound::Jump);
        }
    }
}
