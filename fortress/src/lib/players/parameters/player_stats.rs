use crate::{
    dimensions::{
        Damage,
        time::Microseconds,
    },
    items::{
        ItemConfig,
        ItemType,
        types::{
            ItemTier1,
            ItemTier2,
        },
    },
    math::RandGen,
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
            },
            ItemType::Tier1(tier1) => {
                match tier1 {
                    ItemTier1::CritChanceBoost => {
                        self.weapon.add_crit_chance_level();
                    },
                    ItemTier1::NormalFiringSpeedBoost => {
                        self.weapon.add_normal_firing_speed_level();
                    },
                }
            },
            ItemType::Tier2(tier2) => {
                match tier2 {
                    ItemTier2::CritMultiplierBoost => {
                        self.weapon.add_crit_multiplier_level();
                    },
                    ItemTier2::SpecialFiringPeriodBoost => {
                        self.weapon.add_special_firing_speed_level();
                    },
                }
            }
        }
    }

    pub fn bullet_damage(&self, config: &PlayerBulletConfig, rng: &mut RandGen) -> Damage {
        self.weapon.bullet_damage(config, rng)
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
