use crate::{
    dimensions::{
        Damage,
        time,
    },
    players::PlayerConfig,
};

pub struct WeaponStats {
    base_bullet_speed: f64,
    bullet_speed_level: usize,

    base_bullet_damage: Damage,
    bullet_damage_level: usize,

    base_bullet_knockback_strength: f64,
    bullet_knockback_strength_level: usize,

    base_firing_period: time::Microseconds,
    firing_period_level: usize,
}

impl WeaponStats {
    pub fn new(config: &PlayerConfig) -> WeaponStats {
        WeaponStats {
            base_bullet_speed: config.bullet_speed,
            bullet_speed_level: 1,

            base_bullet_damage: Damage::new(config.bullet_damage),
            bullet_damage_level: 1,

            base_bullet_knockback_strength: config.bullet_knockback_strength,
            bullet_knockback_strength_level: 1,

            base_firing_period: time::milliseconds(config.firing_period_ms),
            firing_period_level: 1,
        }
    }

    pub fn get_bullet_speed(&self) -> f64 {
        self.base_bullet_speed * (self.bullet_speed_level as f64)
    }

    pub fn get_firing_period(&self) -> time::Microseconds {
        self.base_firing_period - (self.firing_period_level as time::Microseconds) * time::milliseconds(5)
    }

    pub fn get_bullet_damage(&self) -> Damage {
        Damage::new(self.base_bullet_damage.value() * (self.bullet_damage_level as i64))
    }

    pub fn get_knockback_strength(&self) -> f64 {
        self.base_bullet_knockback_strength * (self.bullet_knockback_strength_level as f64)
    }
}
