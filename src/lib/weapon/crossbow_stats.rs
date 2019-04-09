use crate::{
    dimensions::{
        Damage,
        time,
    },
    players::PlayerConfig,
};

pub struct CrossbowStats {
    base_arrow_speed: Vec2,
    pub arrow_speed_level: usize,

    base_arrow_damage: Damage,
    pub arrow_damage_level: usize,

    base_arrow_knockback_strength: f32,
    pub arrow_knockback_strength_level: usize,

    base_firing_period: time::Microseconds,
    pub firing_period_level: usize,
}

impl CrossbowStats {
    pub fn new(config: &PlayerConfig) -> CrossbowStats {
        CrossbowStats {
            base_arrow_speed: Vec2::new(config.arrow_speed.0, config.arrow_speed.1),
            arrow_speed_level: 1,

            base_arrow_damage: config.arrow_damage,
            arrow_damage_level: 1,

            base_arrow_knockback_strength: config.arrow_knockback_strength,
            arrow_knockback_strength_level: 1,

            base_firing_period: time::milliseconds(config.firing_period_ms),
            firing_period_level: 1,
        }
    }

    pub fn get_arrow_speed(&self) -> Vec2 {
        Vec2::new(self.base_arrow_speed.x * (self.arrow_speed_level as f32), self.base_arrow_speed.y)
    }

    pub fn get_firing_period(&self) -> time::Microseconds {
        self.base_firing_period - (self.firing_period_level as time::Microseconds) * time::milliseconds(5)
    }

    pub fn get_arrow_damage(&self) -> Damage {
        self.base_arrow_damage * (self.arrow_damage_level as Damage)
    }

    pub fn get_knockback_strength(&self) -> f32 {
        self.base_arrow_knockback_strength * (self.arrow_knockback_strength_level as f32)
    }
}
