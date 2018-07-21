use app::StatusOr;
use controls::KeyboardControls;
use dimensions::time::DeltaTime;
use gl;
use render::{
    Camera,
    GBuffer,
};
use sdl2::render::WindowCanvas;

pub struct RenderState {
    camera: Camera,
    g_buffer: GBuffer,
}

impl RenderState {
    pub fn new(width: i32, height: i32) -> StatusOr<RenderState> {
        unsafe { gl::Enable(gl::DEPTH_TEST); }
        let mut render = RenderState {
            camera: Camera::new(),
            g_buffer: GBuffer::new()?,
        };
        render.resize(width, height)?;
        Ok(render)
    }

    pub fn update(&mut self, keyboard: &KeyboardControls, dt: DeltaTime) {
        self.camera.update(keyboard, dt);
    }

    pub fn resize(&mut self, width: i32, height: i32) -> StatusOr<()> {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
        self.g_buffer.gl_init(width, height)?;
        Ok(())
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        unsafe {
            gl::ClearColor(0.0177, 0.0177, 0.0477, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 1. Draw all geometry.
        self.g_buffer.geometry_pass(); {
            // let projection_view = self.camera.projection(canvas) * self.camera.view();
        }

        // 2. Lighting pass
        self.g_buffer.lighting_pass();

        // 3. Non-geometric superimposed draw calls.

        canvas.present();
    }
}