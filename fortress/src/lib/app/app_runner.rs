use app::{
    Clock,
    StatusOr,
};
use controls::KeyboardControls;
use gl;
use render::RenderState;
use sdl2::{
    event::{
        Event,
        WindowEvent,
    },
    keyboard::Keycode,
    render::WindowCanvas,
    self,
};

fn find_sdl_gl_driver() -> StatusOr<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Ok(index as u32);
        }
    }
    Err(String::from("Could not find SDL GL driver."))
}

pub struct AppRunner {
    _sdl_context: sdl2::Sdl,
    _gl_context: sdl2::video::GLContext,
    _video_subsystem: sdl2::VideoSubsystem,
    canvas: WindowCanvas,
    events: sdl2::EventPump,
    clock: Clock,
    render: RenderState,
    keyboard: KeyboardControls,
}

impl AppRunner {
    pub fn new() -> StatusOr<AppRunner> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        {
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_depth_size(24);
            gl_attr.set_context_version(4, 5);
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        }

        let window_size: (i32, i32) = (1200, 670);
        let window = video_subsystem.window("App", window_size.0 as u32, window_size.1 as u32)
            .opengl()
            .build()
            .map_err(|err| format!("Error initializing window: {}", err))?;

        let gl_context = window.gl_create_context()?;
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        video_subsystem.gl_set_swap_interval(1);

        let canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver()?)
            .build()
            .map_err(|err| format!("Error initializing canvas: {}", err))?;
        canvas.window().gl_set_context_to_current()?;

        let render = RenderState::new(window_size.0, window_size.1)?;

        let events = sdl_context.event_pump()?;
        Ok(AppRunner {
            _sdl_context: sdl_context,
            _gl_context: gl_context,
            _video_subsystem: video_subsystem,
            canvas,
            events,
            clock: Clock::start(),
            render,
            keyboard: KeyboardControls::new(),
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
                }
            }
        }
    }

    // Return false on quit.
    fn process_events(&mut self) -> StatusOr<bool> {
        for event in self.events.poll_iter() {
           match event {
               Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Q), ..} => return Ok(false),
               Event::Window { win_event: WindowEvent::Resized(width, height), .. } =>
                   self.render.resize(width, height)?,
               _ => ()
           }
        }
        Ok(true)
    }

    fn update(&mut self) {
        let dt = self.clock.restart();
        self.keyboard.update(&self.events);
        self.render.update(&self.keyboard, dt);
    }

    fn draw(&mut self) {
        self.render.draw(&mut self.canvas);
    }
}
