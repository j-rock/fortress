use crate::{
    app::{
        AppContext,
        Clock,
        RandGen,
        StatusOr,
    },
    audio::AudioPlayer,
    control::Controller,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    world::WorldState,
};
use gl;
use glm;
use sdl2::{
    event::{
        Event,
        WindowEvent,
    },
    keyboard::Keycode,
};

#[derive(Deserialize)]
struct AppRunnerConfig {
    window_size: (i32, i32),
    sleep_to_frame_micros: i64,
}

pub struct AppRunner {
    audio: AudioPlayer,
    clock: Clock,
    controller: Controller,
    rng: RandGen,
    world: WorldState,
    config_watcher: ConfigWatcher,
    config: SimpleConfigManager<AppRunnerConfig>,

    // Declare AppContext last so its dropped last.
    context: AppContext,
}

impl AppRunner {
    pub fn new() -> StatusOr<AppRunner> {
        let mut config_watcher = ConfigWatcher::new()?;

        let config = SimpleConfigManager::from_config_resource(&mut config_watcher, "app.conf")?;

        let context = {
            let config: &AppRunnerConfig = config.get();
            AppContext::new(config.window_size)?
        };
        let audio = AudioPlayer::new(&mut config_watcher)?;
        let controller = Controller::new(&mut config_watcher)?;
        let world = WorldState::new(&mut config_watcher)?;

        Ok(AppRunner {
            audio,
            clock: Clock::start(),
            controller,
            rng: RandGen::new(),
            world,
            config_watcher,
            config,
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

                    let dt_micros = self.clock.restart().as_microseconds();
                    if dt_micros < self.config.get().sleep_to_frame_micros {
                        let sleep_time_micros = self.config.get().sleep_to_frame_micros - dt_micros;
                        let sleep_duration = std::time::Duration::from_micros(sleep_time_micros as u64);
                        std::thread::sleep(sleep_duration);
                    }
                }
            }
        }
    }

    // Return false on quit.
    fn process_events(&mut self) -> StatusOr<bool> {
        let mut gamepad_events = Vec::new();
        for event in self.context.events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Q), ..} => return Ok(false),
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
        let screen_size = self.screen_size();
        let color = self.world.clear_color();
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 1. Draw all geometry.
        self.world.draw(screen_size);

        // 2. Non-geometric superimposed draw calls.
    }

    fn screen_size(&self) -> glm::IVec2 {
        let (x, y) = self.context.canvas.window().size();
        glm::ivec2(x as i32, y as i32)
    }
}
