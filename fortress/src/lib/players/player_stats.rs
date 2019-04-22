use crate::players::PlayerConfig;

pub struct PlayerStats {
    base_move_speed: f64,
    move_speed_level: usize,
}

impl PlayerStats {
    pub fn new(config: &PlayerConfig) -> PlayerStats {
        PlayerStats {
            base_move_speed: config.base_move_speed,
            move_speed_level: 1,
        }
    }

    pub fn get_move_speed(&self) -> f64 {
        self.base_move_speed * (self.move_speed_level as f64)
    }
}
