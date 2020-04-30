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
    crit_chance_level: usize,
    crit_multiplier_level: usize,
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
            crit_chance_level: 1,
            crit_multiplier_level: 1,
            knockback_level: 1,
            normal_firing_speed_level: 1,
            special_firing_speed_level: 1,
        }
    }
}

impl WeaponParameters {
    pub fn add_crit_chance_level(&mut self) {
        self.crit_chance_level += 1;
    }

    pub fn add_crit_multiplier_level(&mut self) {
        self.crit_multiplier_level += 1;
    }

    pub fn bullet_damage(&self, config: &PlayerBulletConfig, rng: &mut RandGen) -> Damage {
        let low_value = self.damage_level as i64 * config.base_damage_per_level;
        let high_value = low_value * config.random_damage_multiplier;
        let damage = rng.ranged_i64(low_value, high_value);

        let uncapped_crit_chance = config.base_crit_chance + config.crit_chance_per_level * self.crit_chance_level as f64;
        let crit_chance = if uncapped_crit_chance < config.max_crit_chance { uncapped_crit_chance } else { config.max_crit_chance };

        if rng.flip_coin(crit_chance) {
            let uncapped_multiplier = config.base_on_crit_damage_multiplier + config.crit_multiplier_per_level * self.crit_multiplier_level as i64;
            let multiplier = if uncapped_multiplier < config.max_on_crit_damage_multiplier {
                uncapped_multiplier
            } else {
                config.max_on_crit_damage_multiplier
            };
            Damage::new(damage * multiplier, Criticality::Crit)
        } else {
            Damage::new(damage, Criticality::Normal)
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
