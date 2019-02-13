use dimensions::Health;
use liquidfun::box2d::common::math::Vec2;
use wraiths::{
    Wraith,
    WraithConfig,
    state::WraithBody
};

pub struct WraithState {
    pub config: WraithConfig,
    pub body: WraithBody,
    pub health: Health,
}

impl WraithState {
    pub fn new(config: WraithConfig, body: WraithBody) -> WraithState {
        let health = config.starting_health;
        WraithState {
            config,
            body,
            health,
        }
    }

    pub fn register(&mut self, wraith: *const Wraith) {
        self.body.register(wraith);
    }

    pub fn get_body_position(&self) -> Vec2 {
        *self.body.body.data_setter.get_position()
    }
}
