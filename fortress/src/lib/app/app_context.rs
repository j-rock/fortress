use crate::app::StatusOr;
use gl;
use glm;
use sdl2;

fn find_sdl_gl_driver() -> StatusOr<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Ok(index as u32);
        }
    }
    Err(String::from("Could not find SDL GL driver."))
}
pub struct AppContext {
    pub events: sdl2::EventPump,
    pub canvas: sdl2::render::WindowCanvas,
    pub controller_subsystem: sdl2::GameControllerSubsystem,
    _video_subsystem: sdl2::VideoSubsystem,
    _gl_context: sdl2::video::GLContext,
    _sdl_context: sdl2::Sdl,
}

impl AppContext {
    pub fn new(window_size: (i32, i32)) -> StatusOr<AppContext> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        {
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_depth_size(24);
            gl_attr.set_context_version(4, 5);
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        }

        let window = video_subsystem.window("App", window_size.0 as u32, window_size.1 as u32)
            .opengl()
            .build()
            .map_err(|err| format!("Error initializing window: {}", err))?;

        let gl_context = window.gl_create_context()?;
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        video_subsystem.gl_set_swap_interval(1)?;

        let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver()?)
            .build()
            .map_err(|err| format!("Error initializing canvas: {}", err))?;
        canvas.window().gl_set_context_to_current()?;
        canvas.window_mut().raise();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Viewport(0, 0, window_size.0, window_size.1);
        }

        let controller_subsystem = sdl_context.game_controller()?;
        controller_subsystem.set_event_state(true);

        let events = sdl_context.event_pump()?;

        Ok(AppContext {
            _sdl_context: sdl_context,
            _gl_context: gl_context,
            _video_subsystem: video_subsystem,
            controller_subsystem,
            canvas,
            events
        })
    }

    pub fn screen_size(&self) -> glm::IVec2 {
        let (x, y) = self.canvas.window().size();
        glm::ivec2(x as i32, y as i32)
    }
}
