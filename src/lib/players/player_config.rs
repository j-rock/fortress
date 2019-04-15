#[derive(Clone, Deserialize)]
pub struct PlayerConfig {
    pub physical_radius: f64,
    pub physical_density: f64,
    pub base_move_speed: f64,

    pub weapon_physical_offset: f64,
    pub bullet_radius: f64,
    pub bullet_speed: f64,
    pub bullet_damage: i64,
    pub bullet_knockback_strength: f64,
    pub firing_period_ms: i64,
}

