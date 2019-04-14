#[derive(Clone, Deserialize)]
pub struct PlayerConfig {
    pub physical_radius: f64,
    pub physical_density: f64,
    pub base_move_speed: f64,

    pub crossbow_body_offset: (f64, f64),
    pub arrow_box_size: (f64, f64),
    pub arrow_speed: (f64, f64),
    pub arrow_damage: i64,
    pub arrow_knockback_strength: f64,
    pub firing_period_ms: i64,
}

