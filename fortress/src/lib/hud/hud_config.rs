#[derive(Deserialize)]
pub struct HudConfig {
    pub frames: FrameCounterConfig,
    pub skulls: SkullCounterConfig,
}

#[derive(Deserialize)]
pub struct FrameCounterConfig {
    pub num_last_frames_to_average: usize,
    pub fps_text_screen_pos: (f32, f32, f32),
    pub color: (f32, f32, f32),
    pub alpha: f32,
}

#[derive(Deserialize)]
pub struct SkullCounterConfig {
    pub screen_pos: (f32, f32, f32),
    pub color: (f32, f32, f32),
    pub alpha: f32,
}
