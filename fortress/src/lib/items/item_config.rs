use crate::items::{
    barrels::BarrelConfig,
    ItemType,
    types::SkullType,
};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ItemConfig {
    pub system_items_initial_capacity: usize,
    pub system_barrels_initial_capacity: usize,
    pub physical_radius: f64,
    pub physical_density: f64,
    pub render_scale: f32,
    pub bloom_intensity: f32,
    pub light_elevation: f32,
    pub light_attenuation: (f32, f32, f32),
    pub item_type_light_color: HashMap<ItemType, (f32, f32, f32)>,
    pub skull_value: HashMap<SkullType, i64>,

    pub barrel: BarrelConfig,
}