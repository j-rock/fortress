#[derive(Deserialize)]
pub struct WorldUiConfig {
    pub frames: FrameCounterConfig,
}

#[derive(Deserialize)]
pub struct FrameCounterConfig {
    pub num_last_frames_to_average: usize,
    pub fps_text_screen_pos: (f32, f32, f32),
    pub num_screen_pos: (f32, f32, f32),
    pub color: (f32, f32, f32),
    pub alpha: f32,
}
