use crate::{
    dimensions::{
        Criticality,
        Damage,
        time::Microseconds,
    },
    math::RandGen,
    players::PlayerBulletConfig,
};

pub struct WeaponParameters {
    bullet_speed_level: usize,
    crit_level: usize,
    damage_level: usize,
    knockback_level: usize,
    normal_firing_speed_level: usize,
    special_firing_speed_level: usize,
}

impl Default for WeaponParameters {
    fn default() -> Self {
        WeaponParameters {
            bullet_speed_level: 1,
            damage_level: 1,
            crit_level: 1,
            knockback_level: 1,
            normal_firing_speed_level: 1,
            special_firing_speed_level: 1,
        }
    }
}

impl WeaponParameters {
    pub fn bullet_damage(&self, config: &PlayerBulletConfig, rng: &mut RandGen) -> Damage {
        let crit_chance = config.base_crit_chance + config.crit_chance_per_level * self.crit_level as f64;
        let apply_crit = if crit_chance < 1.0 {
            rng.flip_coin(crit_chance)
        } else {
            true
        };

        let low_value = self.damage_level as i64 * config.base_damage_per_level;
        let high_value = low_value * config.random_damage_multiplier;
        let rand = rng.ranged_i64(low_value, high_value);
        if apply_crit {
            Damage::new(rand * config.on_crit_damage_multiplier, Criticality::Crit)
        } else {
            Damage::new(rand, Criticality::Normal)
        }
    }

    pub fn bullet_knockback(&self, config: &PlayerBulletConfig) -> f64 {
        self.knockback_level as f64 * config.knockback_strength
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
