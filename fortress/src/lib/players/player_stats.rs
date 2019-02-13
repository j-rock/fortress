use players::PlayerConfig;

pub struct PlayerStats {
    move_speed: f32,
    pub move_speed_level: usize,

    jump_strength: f32,
    pub jump_strength_level: usize,

    num_jumps: i32,
    pub num_jumps_level: usize,
}

impl PlayerStats {
    pub fn new(config: &PlayerConfig) -> PlayerStats {
        PlayerStats {
            move_speed: config.move_speed,
            move_speed_level: 1,

            jump_strength: config.jump_strength,
            jump_strength_level: 1,

            num_jumps: config.num_jumps,
            num_jumps_level: 1,
        }
    }

    pub fn get_move_speed(&self) -> f32 {
        self.move_speed * (self.move_speed_level as f32)
    }

    pub fn get_jump_strength(&self) -> f32 {
        self.jump_strength * (self.jump_strength_level as f32)
    }

    pub fn get_num_jumps(&self) -> i32 {
        self.num_jumps + (self.num_jumps_level as i32 - 1)
    }
}
