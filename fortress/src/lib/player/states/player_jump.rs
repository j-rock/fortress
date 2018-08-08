use dimensions::{
    LrDirection,
    time::{
        self,
        DeltaTime,
    }
};
use entity::{
    EntityRegistrar,
    EntityType,
    Registered,
};
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::fixture::{
        Fixture,
        FixtureDef
    },
};
use player::PlayerBody;

pub struct PlayerJumpConfig {
    jump_strength: f32,
    max_jumps: i32,
    delay_after_jump: time::Microseconds,
}

pub struct PlayerJump {
    player_body: PlayerBody,

    jump_strength: f32,
    jumps_left: i32,
    max_jumps: i32,
    delay_after_jump: time::Microseconds,
    current_delay: time::Microseconds,
}

impl PlayerState for PlayerJump {
    fn update(mut self, controller: &Controller, registrar: &mut EntityRegistrar, dt: DeltaTime) -> Box<dyn PlayerState> {
        self.foot_sensor.register(registrar, player);
        self.current_delay -= dt.as_microseconds();
        if self.current_delay < 0 {
            self.current_delay = 0;
        }

        Box::new(self)
    }
}

impl PlayerJump {
    pub fn new(config: PlayerJumpConfig, player_body: PlayerBody) -> PlayerJump {
        let foot_sensor = Self::create_foot_sensor(&config, &player_body);

        PlayerJump {
            player_body,
            foot_sensor,
            jump_strength: config.jump_strength,
            jumps_left: config.max_jumps,
            max_jumps: config.max_jumps,
            delay_after_jump: time::milliseconds(config.delay_after_jump),
            current_delay: time::milliseconds(0),
        }
    }

    pub fn try_jump(&mut self) {
        if self.jumps_left > 0 && self.current_delay == 0 {
            self.current_delay = self.delay_after_jump;
            self.jumps_left -= 1;

            let body = self.player_body.body();
            let actual_body_velocity = *body.get_linear_velocity();
            let jump_boost = self.jump_strength - actual_body_velocity.y;
            let mass = body.get_mass();
            let impulse = liquidfun::box2d::common::math::Vec2::new(0.0, mass * jump_boost);
            let body_center = *body.get_world_center();
            body.apply_linear_impulse(&impulse, &body_center, true);
        }
    }

    pub fn make_foot_contact(&mut self) {
        // TODO: Transiiton to Upright state
        self.jumps_left = self.max_jumps;
    }

    fn create_foot_sensor(config: &FootSensorConfig, player_body: &PlayerBody) -> Registered<Fixture> {
    }
}
