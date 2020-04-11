use crate::{
    app::StatusOr,
    data::RingBufferView,
    dimensions::time::DeltaTime,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    text::{
        NamedText,
        RasterSize,
        TextContent,
        TextRenderer,
        TextRenderRequest,
    },
    world::{
        FrameCounterConfig,
        WorldUiConfig,
    },
};

pub struct WorldUi {
    config: SimpleConfigManager<WorldUiConfig>,
    frames: FrameCounter,
}

impl WorldUi {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::<WorldUiConfig>::from_config_resource(config_watcher, "world_ui.conf")?;

        let frames = {
            let config = config.get();
            FrameCounter::new(&config.frames)
        };

        Ok(WorldUi {
            config,
            frames,
        })
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        if self.config.update() {
            let config = self.config.get();
            self.frames = FrameCounter::new(&config.frames);
        }

        self.frames.pre_update(dt);
    }

    pub fn queue_draw(&self, text: &mut TextRenderer) {
        let config = self.config.get();
        self.frames.queue_draw(&config.frames, text);
    }
}

struct FrameCounter {
    last_n_frame_seconds: Vec<f64>,
    ring_buffer_view: RingBufferView,
}

impl FrameCounter {
    pub fn new(config: &FrameCounterConfig) -> Self {
        FrameCounter {
            last_n_frame_seconds: Vec::with_capacity(config.num_last_frames_to_average),
            ring_buffer_view: RingBufferView::with_capacity(config.num_last_frames_to_average),
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.ring_buffer_view.add_element_at_head(dt.as_f64_seconds(), &mut self.last_n_frame_seconds);
        self.ring_buffer_view.increment_head();
    }

    pub fn queue_draw(&self, config: &FrameCounterConfig, text: &mut TextRenderer) {
        let num_frames_collected = self.last_n_frame_seconds.len();
        let fps = if num_frames_collected > 0 {
            let frame_second_sum: f64 = self.last_n_frame_seconds.iter().sum();
            (num_frames_collected as f64 / frame_second_sum).round() as i64
        } else {
            0
        };

        text.queue(TextRenderRequest {
            content: TextContent::Text(NamedText::FpsPrefix),
            screen_position_percentage: glm::vec3(config.fps_text_screen_pos.0, config.fps_text_screen_pos.1, config.fps_text_screen_pos.2),
            raster_size: RasterSize::Small,
            color: glm::vec3(config.color.0, config.color.1, config.color.2),
            alpha: config.alpha
        });
        text.queue(TextRenderRequest {
            content: TextContent::Number(fps),
            screen_position_percentage: glm::vec3(config.num_screen_pos.0, config.num_screen_pos.1, config.num_screen_pos.2),
            raster_size: RasterSize::Small,
            color: glm::vec3(config.color.0, config.color.1, config.color.2),
            alpha: config.alpha
        })
    }
}
