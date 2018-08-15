use liquidfun::box2d::common::math::Vec2;
use wraith::{
    WraithConfig,
    state::WraithBody
};

pub struct WraithState {
    pub config: WraithConfig,
    pub body: WraithBody,
}

impl WraithState {
    pub fn new(config: WraithConfig, body: WraithBody) -> WraithState {
        WraithState {
            config,
            body
        }
    }

    pub fn get_body_position(&self) -> Vec2 {
        *self.body.body.get_position()
    }
}
