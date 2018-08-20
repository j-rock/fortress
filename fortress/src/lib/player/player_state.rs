use crossbow::Crossbow;
use dimensions::time::DeltaTime;
use entity::EntityRegistrar;
use liquidfun::box2d::{
    common::math::Vec2,
    dynamics::world::World,
};
use player::{
    Player,
    PlayerConfig,
    state::{
        PlayerBody,
        SlashState,
    },
};

pub struct PlayerState {
    pub config: PlayerConfig,
    pub body: PlayerBody,
    pub slash: SlashState,
    pub crossbow: Crossbow,
}

impl PlayerState {
    pub fn new(config: PlayerConfig, registrar: &EntityRegistrar, world: &mut World) -> PlayerState {
        let body = PlayerBody::new(&config, registrar, world);
        let slash = SlashState::new(&config);
        let crossbow = Crossbow::new(&config, registrar, world);
        PlayerState {
            config,
            body,
            slash,
            crossbow,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        self.body.register(player);
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.slash.pre_update(&mut self.body, dt);
    }

    pub fn try_slash(&mut self) {
        self.slash.try_slash(&mut self.body);
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