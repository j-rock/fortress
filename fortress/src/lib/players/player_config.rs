#[derive(Clone, Deserialize)]
pub struct PlayerConfig {
    pub physical_radius: f64,
    pub physical_density: f64,
    pub base_move_speed: f64,
    pub weapon_physical_offset: f64,
    pub firing_period_ms: i64,
    pub player_render_offset: (f32, f32),
    pub player_render_scale: (f32, f32),
    pub player_idle_frame_duration_micros: i64,
    pub player_running_frame_duration_micros: i64,

    pub bullet_lifetime_duration_micros: i64,
    pub bullet_physical_radius: f64,
    pub bullet_speed: f64,
    pub bullet_damage: i64,
    pub bullet_knockback_strength: f64,
    pub bullet_render_width: f32,
    pub bullet_render_height: f32,
    pub bullet_render_elevation: f32,
    pub bullet_light_color: (f32, f32, f32),
    pub bullet_light_attenuation: (f32, f32, f32),
    pub bullet_sprite_frame_duration_micros: i64,

    pub item_collection_animation_num_concurrent_guess: usize,
    pub item_collection_animation_duration_micros: i64,
    pub item_collection_render_radius: f32,
    pub item_collection_animation_spin_radius: f32,
    pub item_collection_animation_spin_max_speed: f32,
    pub item_collection_animation_max_height: f32,
    pub item_collection_attenuation: (f32, f32, f32),
}

