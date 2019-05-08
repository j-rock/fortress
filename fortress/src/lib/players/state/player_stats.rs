use crate::players::PlayerConfig;

pub struct PlayerStats {
    base_move_speed: f64,
    move_speed_level: usize,

    skulls_collected: usize,
}

impl PlayerStats {
    pub fn new(config: &PlayerConfig) -> PlayerStats {
        PlayerStats {
            base_move_speed: config.base_move_speed,
            move_speed_level: 1,
            skulls_collected: 0,
        }
    }

    pub fn get_move_speed(&self) -> f64 {
        self.base_move_speed * (self.move_speed_level as f64)
    }

    pub fn collect_mega_skull(&mut self) {
        self.skulls_collected += 5;
    }

    pub fn collect_skull(&mut self) {
        self.skulls_collected += 1;
    }
}
