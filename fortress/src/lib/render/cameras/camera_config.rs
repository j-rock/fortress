#[derive(Deserialize)]
pub struct CameraConfig {
    pub zoom: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub position: (f32, f32, f32),
    pub lookat: (f32, f32, f32),
    pub right: (f32, f32, f32)
}

