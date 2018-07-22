use app::{
    AppContext,
    Clock,
    StatusOr,
};
use controls::Controller;
use file::ConfigWatcher;
use gl;
use render::GBuffer;
use sdl2::{
    event::{
        Event,
        WindowEvent,
    },
    keyboard::Keycode,
};
use world::WorldState;

pub struct AppRunner {
    context: AppContext,
    clock: Clock,
    config_watcher: ConfigWatcher,
    controller: Controller,
    g_buffer: GBuffer,
    world: WorldState,
}

impl AppRunner {
    pub fn new() -> StatusOr<AppRunner> {
        let window_size: (i32, i32) = (1200, 670);
        let mut config_watcher = ConfigWatcher::new()?;
        let world = WorldState::new(&mut config_watcher)?;

        Ok(AppRunner {
            context: AppContext::new(&window_size)?,
            clock: Clock::start(),
            config_watcher,
            controller: Controller::new(),
            g_buffer: GBuffer::new(&window_size)?,
            world,
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
                   self.g_buffer.resize(width, height)?
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

    fn draw(&self) {
        unsafe {
            gl::ClearColor(0.0177, 0.0177, 0.0477, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 1. Draw all geometry.
        self.g_buffer.geometry_pass();
        self.world.draw_geometry();

        // 2. Lighting pass
        self.g_buffer.lighting_pass();

        // 3. Non-geometric superimposed draw calls.
    }
}
