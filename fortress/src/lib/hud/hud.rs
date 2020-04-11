use crate::{
    app::StatusOr,
    dimensions::time::DeltaTime,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    hud::{
        FrameCounter,
        HudConfig,
    },
    text::TextRenderer,
};

pub struct Hud {
    config: SimpleConfigManager<HudConfig>,
    frames: FrameCounter,
}

impl Hud {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::<HudConfig>::from_config_resource(config_watcher, "hud.conf")?;

        let frames = {
            let config = config.get();
            FrameCounter::new(&config.frames)
        };

        Ok(Hud {
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
