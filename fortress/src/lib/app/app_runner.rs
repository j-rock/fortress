use crate::{
    app::{
        AppContext,
        AppRunnerConfig,
        Clock,
        StatusOr,
    },
    audio::AudioPlayer,
    control::Controller,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    math::RandGen,
    render::BloomPipeline,
    world::WorldState,
};
use gl;
use sdl2::{
    event::{
        Event,
        WindowEvent,
    },
    keyboard::Keycode,
};

pub struct AppRunner {
    audio: AudioPlayer,
    clock: Clock,
    controller: Controller,
    rng: RandGen,
    world: WorldState,
    config_watcher: ConfigWatcher,
    config: SimpleConfigManager<AppRunnerConfig>,
    bloom_render_pipeline: BloomPipeline,

    // Declare AppContext last so its dropped last.
    context: AppContext,
}

impl AppRunner {
    pub fn new() -> StatusOr<AppRunner> {
        let mut config_watcher = ConfigWatcher::new()?;

        let config: SimpleConfigManager<AppRunnerConfig> = SimpleConfigManager::from_config_resource(&mut config_watcher, "app.conf")?;

        let context = {
            let config = config.get();
            AppContext::new(config.app.window_size)?
        };
        let audio = AudioPlayer::new(&mut config_watcher)?;
        let controller = Controller::new(&mut config_watcher)?;
        let mut rng = RandGen::new();
        let world = WorldState::new(&mut config_watcher, &mut rng)?;

        let bloom_render_pipeline = BloomPipeline::new(context.screen_size())?;

        Ok(AppRunner {
            audio,
            clock: Clock::start(),
            controller,
            rng,
            world,
            config_watcher,
            config,
            bloom_render_pipeline,
            context,
        })
    }

    pub fn run(&mut self) -> StatusOr<()> {
        let _ = self.clock.restart();
        loop {
            match self.process_events() {
                Err(e) => return Err(e),
                Ok(false) => return Ok(()),
                _ => {
                    self.update();
                    self.draw();
                    self.context.canvas.present();

                    let ref config = self.config.get().app;
                    let dt_micros = self.clock.peek().as_microseconds();
                    if dt_micros < config.sleep_to_frame_micros {
                        let sleep_time_micros = config.sleep_to_frame_micros - dt_micros;
                        let sleep_duration = std::time::Duration::from_micros(sleep_time_micros as u64);
                        std::thread::sleep(sleep_duration);
                    }
                }
            }
        }
    }

    // Return false on quit.
    fn process_events(&mut self) -> StatusOr<bool> {
        let ref config = self.config.get().app;
        let mut gamepad_events = Vec::new();
        for event in self.context.events.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(false),
                Event::KeyDown {keycode: Some(Keycode::Q), ..} if config.enable_quit => return Ok(false),
                Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                    unsafe { gl::Viewport(0, 0, width, height); }
                },
                  Event::ControllerDeviceAdded {..}
                | Event::ControllerDeviceRemoved {..}
                | Event::ControllerAxisMotion {..}
                | Event::ControllerButtonDown {..}
                | Event::ControllerButtonUp {..}
                | Event::ControllerDeviceRemapped {..} => {
                      gamepad_events.push(event);
                  },
                _ => {}
           }
        }
        self.controller.ingest_gamepad_events(&self.context.controller_subsystem, gamepad_events);
        Ok(true)
    }

    fn update(&mut self) {
        let dt = self.clock.restart();
        self.config_watcher.update();
        self.config.update();
        self.controller.update(&self.context.events);
        self.audio.update();
        self.world.update(&self.audio, &self.controller, &mut self.rng, dt);
    }

    fn draw(&mut self) {
        let screen_size = self.context.screen_size();
        let color = self.world.clear_color();
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 1. Draw all geometry.
        self.bloom_render_pipeline.begin();
        {
            self.world.draw(screen_size);
        }
        self.bloom_render_pipeline.blur(&self.config.get().bloom);

        // 2. Non-geometric superimposed draw calls.
    }
}
