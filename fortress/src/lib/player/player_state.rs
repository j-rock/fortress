use liquidfun::box2d::common::math::Vec2;
use player::{
    Player,
    PlayerBody,
    PlayerConfig,
};

pub struct PlayerState {
    pub config: PlayerConfig,
    pub body: PlayerBody,
}

impl PlayerState {
    pub fn new(config: PlayerConfig, body: PlayerBody) -> PlayerState {
        PlayerState {
            config,
            body,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        self.body.register(player);
    }

    pub fn get_body_position(&self) -> Vec2 {
        *self.body.body.get_position()
    }

    pub fn get_sword_position(&self) -> Vec2 {
        let body_pos = self.get_body_position();
        let sword_offset = self.body.sword_offset_from_body;
        Vec2 {
            x: body_pos.x + sword_offset.x,
            y: body_pos.y + sword_offset.y
        }
    }
}