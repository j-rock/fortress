#[derive(Deserialize)]
pub struct PlayerConfig {
    pub size: (i32, i32),
    pub spawn_location: (i32, i32),
    pub player_speed: f32,
    // Between [0, 1]
    pub restitution: f32,

    pub jump_strength: f32,
    pub num_jumps: i32,
    pub jump_delay_ms: i64,
}

