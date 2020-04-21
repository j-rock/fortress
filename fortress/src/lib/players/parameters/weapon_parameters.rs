use crate::{
    dimensions::{
        Damage,
        time::{
            self,
            Microseconds,
        }
    },
    players::PlayerBulletConfig,
};

pub struct WeaponParameters {
    bullet_damage_level: usize,
    bullet_knockback_level: usize,
    bullet_speed_level: usize,
    normal_firing_speed_level: usize,
    special_firing_speed_level: usize,
}

impl Default for WeaponParameters {
    fn default() -> Self {
        WeaponParameters {
            bullet_damage_level: 1,
            bullet_knockback_level: 1,
            bullet_speed_level: 1,
            normal_firing_speed_level: 1,
            special_firing_speed_level: 1,
        }
    }
}

impl WeaponParameters {
    pub fn bullet_damage(&self, config: &PlayerBulletConfig) -> Damage {
        let value = self.bullet_damage_level as i64 * config.damage;
        Damage::new(value)
    }

    pub fn bullet_knockback(&self, config: &PlayerBulletConfig) -> f64 {
        self.bullet_knockback_level as f64 * config.knockback_strength
    }

    pub fn bullet_speed(&self, config: &PlayerBulletConfig) -> f64 {
        self.bullet_speed_level as f64 * config.speed
    }

    pub fn normal_firing_period(&self, config: &PlayerBulletConfig) -> Microseconds {
        config.normal_firing_period_micros - (self.normal_firing_speed_level as Microseconds) * time::milliseconds(5)
    }

    pub fn special_firing_period(&self, config: &PlayerBulletConfig) -> Microseconds {
        config.special_firing_period_micros - (self.special_firing_speed_level as Microseconds) * time::milliseconds(5)
    }
}
