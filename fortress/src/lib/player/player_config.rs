#[derive(Deserialize)]
pub struct PlayerConfig {
    pub size: (i32, i32),
    pub spawn_location: (i32, i32),
    pub move_speed: f32,
    pub restitution: f32, // Between [0, 1]

    pub jump_strength: f32,
    pub num_jumps: i32,
    pub jump_delay_ms: i64,

    pub foot_sensor_size: (f32, f32),
    pub foot_sensor_center: (f32, f32)
}

