use crate::text::RasterSize;

#[derive(Deserialize)]
pub struct DamageTextWriterConfig {
    pub initial_capacity: usize,
    pub start_velocity: (f32, f32, f32),
    pub start_height: f32,
    pub vertical_acceleration: f32,
    pub text_expiry_duration_micros: i64,
    pub raster_size: RasterSize,
    pub color: (f32, f32, f32),
    pub shadow_color: (f32, f32, f32),
    pub shadow_offset: (f32, f32, f32),
}