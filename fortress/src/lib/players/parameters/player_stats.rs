use crate::{
    dimensions::{
        Damage,
        time::Microseconds,
    },
    items::{
        ItemConfig,
        ItemType,
    },
    players::{
        parameters::{
            MovementParameters,
            SkullParameters,
            WeaponParameters,
        },
        PlayerBulletConfig,
        PlayerHeroConfig,
    },
};

pub struct PlayerStats {
    movement: MovementParameters,
    weapon: WeaponParameters,
    skull: SkullParameters,
}

impl PlayerStats {
    pub fn new() -> PlayerStats {
        PlayerStats {
            movement: MovementParameters::default(),
            weapon: WeaponParameters::default(),
            skull: SkullParameters::default(),
        }
    }

    pub fn collect_item(&mut self, config: &ItemConfig, item_type: ItemType) {
        match item_type {
            ItemType::Skull(skull_type) => {
                self.skull.add_to_count(config, skull_type);
            }
        }
    }

    pub fn bullet_damage(&self, config: &PlayerBulletConfig) -> Damage {
        self.weapon.bullet_damage(config)
    }

    pub fn bullet_knockback(&self, config: &PlayerBulletConfig) -> f64 {
        self.weapon.bullet_knockback(config)
    }

    pub fn bullet_speed(&self, config: &PlayerBulletConfig) -> f64 {
        self.weapon.bullet_speed(config)
    }

    pub fn normal_firing_period(&self, config: &PlayerBulletConfig) -> Microseconds {
        self.weapon.normal_firing_period(config)
    }

    pub fn special_firing_period(&self, config: &PlayerBulletConfig) -> Microseconds {
        self.weapon.special_firing_period(config)
    }

    pub fn move_speed(&self, config: &PlayerHeroConfig) -> f64 {
        self.movement.move_speed(config)
    }

    pub fn skull_count(&self) -> i64 {
        self.skull.current_count()
    }
}
