use dimensions::time;
use liquidfun;
use player::PlayerConfig;

pub struct JumpTracker {
    jump_strength: f32,
    jumps_left: i32,
    max_jumps: i32,
    delay_after_jump: time::Microseconds,
    current_delay: time::Microseconds,
}

impl JumpTracker {
    pub fn new(config: &PlayerConfig) -> JumpTracker {
        JumpTracker {
            jump_strength: config.jump_strength,
            jumps_left: config.num_jumps,
            max_jumps: config.num_jumps,
            delay_after_jump: time::milliseconds(config.jump_delay_ms),
            current_delay: time::milliseconds(0),
        }
    }

    pub fn make_foot_contact(&mut self) {
        self.jumps_left = self.max_jumps;
    }

    pub fn try_jump(&mut self, player_body: &liquidfun::box2d::dynamics::body::Body) {
        if self.jumps_left > 0 && self.current_delay == 0 {
            self.current_delay = self.delay_after_jump;
            self.jumps_left -= 1;

            let actual_body_velocity = *player_body.get_linear_velocity();
            let jump_boost = self.jump_strength - actual_body_velocity.y;
            let mass = player_body.get_mass();
            let impulse = liquidfun::box2d::common::math::Vec2::new(0.0, mass * jump_boost);
            let body_center = *player_body.get_world_center();
            player_body.apply_linear_impulse(&impulse, &body_center, true);
        }
    }

    pub fn update(&mut self, dt: time::DeltaTime) {
        self.current_delay -= dt.as_microseconds();
        if self.current_delay < 0 {
            self.current_delay = 0;
        }
    }
}
