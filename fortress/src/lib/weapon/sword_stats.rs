use dimensions::{
    Damage,
    time,
};
use players::PlayerConfig;

#[derive(Copy, Clone)]
pub struct SwordStats {
    damage: Damage,
    pub damage_level: usize,

    knockback_strength: f32,
    pub knockback_strength_level: usize,

    period: time::Microseconds,
    pub period_level: usize,
}

impl SwordStats {
    pub fn new(config: &PlayerConfig) -> SwordStats {
        SwordStats {
            damage: config.sword_damage,
            damage_level: 1,

            knockback_strength: config.sword_knockback_strength,
            knockback_strength_level: 1,

            period: time::milliseconds(config.sword_period_ms),
            period_level: 1,
        }
    }

    pub fn get_damage(&self) -> Damage {
        self.damage * (self.damage_level as Damage)
    }

    pub fn get_knockback_strength(&self) -> f32 {
        self.knockback_strength * (self.knockback_strength_level as f32)
    }

    pub fn get_period(&self) -> time::Microseconds {
        self.period - (self.period_level as time::Microseconds) * time::milliseconds(5)
    }
}
