use crate::render::BloomConfig;

#[derive(Deserialize)]
pub struct AppRunnerConfig {
    pub app: AppConfig,
    pub bloom: BloomConfig,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub window_size: (i32, i32),
    pub sleep_to_frame_micros: i64,
    pub enable_quit: bool,
}
