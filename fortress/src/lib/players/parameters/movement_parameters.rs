use crate::players::PlayerHeroConfig;

pub struct MovementParameters {
    move_speed_level: usize,
}

impl Default for MovementParameters {
    fn default() -> Self {
        MovementParameters {
            move_speed_level: 0,
        }
    }
}

impl MovementParameters {
    pub fn move_speed(&self, config: &PlayerHeroConfig) -> f64 {
        config.base_move_speed + self.move_speed_level as f64
    }
}