use crate::{
    audio::{
        AudioPlayer,
        Sound,
    },
    control::{
        Controller,
        ControllerId,
        ControlEvent::{
            PlayerFire,
            PlayerJump,
            PlayerMove,
            PlayerSlash,
        }
    },
    dimensions::{
        LrDirection,
        time::{
            self,
            DeltaTime,
        }
    },
    players::{
        PlayerState,
        state::{
            PlayerStateMachine,
            PlayerUpright,
        }
    },
};
use liquidfun::box2d::common::math::Vec2;

pub struct PlayerJumping {
    jumps_left: i32,
    current_delay: time::Microseconds,
    last_y_coords: LastYCoords,
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
        player_state.body.move_horizontal(player_state.stats.get_move_speed(), move_dir);

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

    fn post_update(&mut self, player_state: &PlayerState, audio: &AudioPlayer) -> Option<Box<dyn PlayerStateMachine>> {
        self.last_y_coords.insert(player_state.get_body_position().y);
        if self.last_y_coords.all_same() {
            audio.play_sound(Sound::Plop);
            Some(Box::new(PlayerUpright::new()))
        } else {
            None
        }
    }
}

impl PlayerJumping {
    pub fn new(player_state: &mut PlayerState, audio: &AudioPlayer) -> PlayerJumping {
        let mut jumping = PlayerJumping {
            jumps_left: player_state.stats.get_num_jumps(),
            current_delay: time::milliseconds(0),
            last_y_coords: LastYCoords::new(player_state)
        };
        jumping.try_jump(player_state, audio);
        jumping
    }

    pub fn try_jump(&mut self, player_state: &mut PlayerState, audio: &AudioPlayer) {
        if self.jumps_left > 0 && self.current_delay == 0 {
            self.current_delay = time::milliseconds(player_state.config.jump_delay_ms);
            self.jumps_left -= 1;

            let body = &player_state.body.body.data_setter;
            let actual_body_velocity = *body.get_linear_velocity();
            let jump_boost = player_state.stats.get_jump_strength() - actual_body_velocity.y;
            let mass = body.get_mass();
            let impulse = Vec2::new(0.0, mass * jump_boost);
            let body_center = *body.get_world_center();
            body.apply_linear_impulse(&impulse, &body_center, true);

            audio.play_sound(Sound::Jump);
        }
    }
}

struct LastYCoords {
    values: Vec<Option<f32>>
}

impl LastYCoords {
    pub fn new(player_state: &PlayerState) -> LastYCoords {
        let mut res = LastYCoords {
            values: vec![None; player_state.config.jump_tracker_num_last_y_coords]
        };
        res.insert(player_state.get_body_position().y);
        res
    }

    pub fn insert(&mut self, next_value: f32) {
        for i in (1 .. self.values.len()).rev() {
            self.values[i] = self.values[i-1];
        }
        self.values[0] = Some(next_value);
    }

    pub fn all_same(&self) -> bool {
        self.values.iter().all(|x| *x == self.values[0])
    }
}

