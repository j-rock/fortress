#[derive(Deserialize)]
pub struct ItemConfig {
    pub items_system_initial_capacity: usize,
    pub item_physical_radius: f64,
    pub item_physical_density: f64,
    pub item_render_scale: f32,
}