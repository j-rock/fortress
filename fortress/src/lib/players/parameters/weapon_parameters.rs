use crate::{
    dimensions::{
        Damage,
        time::Microseconds,
    },
    math::RandGen,
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
    pub fn bullet_damage(&self, config: &PlayerBulletConfig, rng: &mut RandGen) -> Damage {
        let low_value = self.bullet_damage_level as i64 * config.base_damage_per_level;
        let high_value = low_value * config.random_damage_multiplier;
        Damage::new(rng.ranged_i64(low_value, high_value))
    }

    pub fn bullet_knockback(&self, config: &PlayerBulletConfig) -> f64 {
        self.bullet_knockback_level as f64 * config.knockback_strength
    }

    pub fn bullet_speed(&self, config: &PlayerBulletConfig) -> f64 {
        self.bullet_speed_level as f64 * config.speed
    }

    pub fn normal_firing_period(&self, config: &PlayerBulletConfig) -> Microseconds {
        let level_speedup = self.normal_firing_speed_level as Microseconds * config.normal_firing_period.per_level_decrease_micros;
        let firing_period = config.normal_firing_period.baseline_micros - level_speedup;
        if firing_period < config.normal_firing_period.shortest_period_micros {
            config.normal_firing_period.shortest_period_micros
        } else {
            firing_period
        }
    }

    pub fn special_firing_period(&self, config: &PlayerBulletConfig) -> Microseconds {
        let level_speedup = self.special_firing_speed_level as Microseconds * config.special_firing_period.per_level_decrease_micros;
        let firing_period = config.special_firing_period.baseline_micros - level_speedup;
        if firing_period < config.special_firing_period.shortest_period_micros {
            config.special_firing_period.shortest_period_micros
        } else {
            firing_period
        }
    }
}
