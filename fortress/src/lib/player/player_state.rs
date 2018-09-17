use dimensions::{
    Damage,
    LrDirection,
    time::DeltaTime
};
use entity::EntityRegistrar;
use liquidfun::box2d::{
    common::math::Vec2,
    dynamics::world::World,
};
use player::{
    Player,
    PlayerId,
    PlayerConfig,
    state::PlayerBody
};
use weapon::{
    Crossbow,
    Sword
};

pub struct PlayerState {
    pub player_id: PlayerId,
    pub config: PlayerConfig,
    pub body: PlayerBody,
    pub sword: Sword,
    pub crossbow: Crossbow,
}

impl PlayerState {
    pub fn new(player_id: PlayerId, config: PlayerConfig, registrar: &EntityRegistrar, world: &mut World) -> PlayerState {
        let body = PlayerBody::new(&config, registrar, world);
        let sword = Sword::new(&config);
        let crossbow = Crossbow::new(&config, registrar, world);
        PlayerState {
            player_id,
            config,
            body,
            sword,
            crossbow,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        self.body.register(player);
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.sword.pre_update(&mut self.body, dt);
        self.crossbow.pre_update(dt);
    }

    pub fn try_slash(&mut self) {
        self.sword.try_slash(&mut self.body);
    }

    pub fn try_fire(&mut self) {
        let curr_pos = self.get_body_position();
        let curr_dir = self.get_facing_dir();
        let offset = self.config.crossbow_body_offset;
        let start_position = match curr_dir {
            LrDirection::Left => Vec2::new(curr_pos.x - offset.0, curr_pos.y + offset.1),
            LrDirection::Right => Vec2::new(curr_pos.x + offset.0, curr_pos.y + offset.1),
        };

        self.crossbow.try_fire(start_position, curr_dir);
    }

    pub fn get_sword_knockback_strength(&self) -> f32 {
        self.sword.get_knockback_strength()
    }

    pub fn get_sword_damage(&self) -> Damage {
        self.sword.get_damage()
    }

    pub fn get_facing_dir(&self) -> LrDirection {
        self.body.facing_dir
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