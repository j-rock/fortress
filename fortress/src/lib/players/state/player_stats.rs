use crate::{
    dimensions::{
        Damage,
        time::{
            self,
            Microseconds
        },
    },
    items::{
        ItemPickup,
        ItemType,
    },
    players::{
        PlayerBulletConfig,
        PlayerHeroConfig,
    },
};

pub struct PlayerStats {
    base_bullet_speed: f64,
    bullet_speed_level: usize,
    base_bullet_damage: Damage,
    bullet_damage_level: usize,
    base_bullet_knockback_strength: f64,
    bullet_knockback_strength_level: usize,
    base_normal_firing_period: Microseconds,
    normal_firing_period_level: usize,
    base_special_firing_period: Microseconds,
    special_firing_period_level: usize,
    skulls_collected: usize,
}

impl PlayerStats {
    pub fn new(config: &PlayerBulletConfig) -> PlayerStats {
        PlayerStats {
            base_bullet_speed: config.speed,
            bullet_speed_level: 1,
            base_bullet_damage: Damage::new(config.damage),
            bullet_damage_level: 1,
            base_bullet_knockback_strength: config.knockback_strength,
            bullet_knockback_strength_level: 1,
            base_normal_firing_period: config.normal_firing_period_micros,
            normal_firing_period_level: 1,
            base_special_firing_period: config.special_firing_period_micros,
            special_firing_period_level: 1,
            skulls_collected: 0,
        }
    }

    pub fn get_bullet_speed(&self) -> f64 {
        self.base_bullet_speed * (self.bullet_speed_level as f64)
    }

    pub fn get_normal_firing_period(&self) -> Microseconds {
        self.base_normal_firing_period - (self.normal_firing_period_level as Microseconds) * time::milliseconds(5)
    }

    pub fn get_special_firing_period(&self) -> Microseconds {
        self.base_special_firing_period - (self.special_firing_period_level as Microseconds) * time::milliseconds(5)
    }

    pub fn get_bullet_damage(&self) -> Damage {
        Damage::new(self.base_bullet_damage.value() * (self.bullet_damage_level as i64))
    }

    pub fn get_knockback_strength(&self) -> f64 {
        self.base_bullet_knockback_strength * (self.bullet_knockback_strength_level as f64)
    }

    pub fn get_move_speed(&self, config: &PlayerHeroConfig) -> f64 {
        config.base_move_speed
    }

    pub fn collect_item(&mut self, item_pickup: ItemPickup) {
        match item_pickup.item_type() {
            ItemType::MegaSkull => {
                self.skulls_collected += 5;
            },
            ItemType::Skull => {
                self.skulls_collected += 1;
            },
        }
    }

    pub fn skull_count(&self) -> usize {
        self.skulls_collected
    }
}
