use app::{
    AppContext,
    Clock,
    StatusOr,
};
use control::Controller;
use file::{
    ConfigLoader,
    ConfigWatcher,
    self
};
use gl;
use glm;
use render::GBuffer;
use sdl2::{
    event::{
        Event,
        WindowEvent,
    },
    keyboard::Keycode,
};
use world::WorldState;

#[derive(Deserialize)]
struct AppRunnerConfig {
    window_size: (i32, i32)
}

pub struct AppRunner {
    clock: Clock,
    controller: Controller,
    g_buffer: GBuffer,
    world: WorldState,
    config_watcher: ConfigWatcher,

    // Declare AppContext last so its dropped last.
    context: AppContext,
}

impl AppRunner {
    pub fn new() -> StatusOr<AppRunner> {
        let mut config_watcher = ConfigWatcher::new()?;

        let config_path = file::util::resource_path("config", "app.conf");
        let config = ConfigLoader::<AppRunnerConfig>::read_config_file(&config_path)?;

        let context = AppContext::new(&config.window_size)?;
        let world = WorldState::new(&mut config_watcher)?;

        Ok(AppRunner {
            config_watcher,
            context,
            clock: Clock::start(),
            controller: Controller::new(),
            g_buffer: GBuffer::new(&config.window_size)?,
            world,
        })
    }

    pub fn run(&mut self) -> StatusOr<()> {
        self.world.register();
        let _ = self.clock.restart();
        loop {
            match self.process_events() {
                Err(e) => return Err(e),
                Ok(false) => return Ok(()),
                _ => {
                    self.update();
                    self.draw();
                    self.context.canvas.present();
                }
            }
        }
    }

    // Return false on quit.
    fn process_events(&mut self) -> StatusOr<bool> {
        for event in self.context.events.poll_iter() {
           match event {
               Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Q), ..} => return Ok(false),
               Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                   unsafe { gl::Viewport(0, 0, width, height); }
                   self.g_buffer.resize(width, height)?;
               },
               _ => ()
           }
        }
        Ok(true)
    }

    fn update(&mut self) {
        let dt = self.clock.restart();
        self.config_watcher.update();
        self.controller.update(&self.context.events);
        self.world.update(&self.controller, dt);
    }

    fn draw(&mut self) {
        let screen_size = self.screen_size();
        let color = self.world.clear_color();
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 1. Draw all geometry.
        self.g_buffer.geometry_pass();
        self.world.draw_geometry(screen_size);

        // 2. Lighting pass
        self.g_buffer.lighting_pass();

        // 3. Non-geometric superimposed draw calls.
    }

    fn screen_size(&self) -> glm::IVec2 {
        let (x, y) = self.context.canvas.window().size();
        glm::ivec2(x as i32, y as i32)
    }
}
