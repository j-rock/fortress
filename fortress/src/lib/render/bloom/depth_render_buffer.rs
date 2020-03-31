use gl::{
    self,
    types::{
        GLsizei,
        GLuint,
    },
};
use glm;

pub struct DepthRenderBuffer {
    render_buffer: GLuint,
}

impl DepthRenderBuffer {
    pub fn new(screen_size: glm::IVec2) -> Self {
        let mut render_buffer: GLuint = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut render_buffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, screen_size.x as GLsizei, screen_size.y as GLsizei);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, render_buffer);
        }
        DepthRenderBuffer {
            render_buffer
        }
    }
}

impl Drop for DepthRenderBuffer {
    fn drop(&mut self) {
        unsafe {
            if self.render_buffer != 0 {
                gl::DeleteRenderbuffers(1, &self.render_buffer);
                self.render_buffer = 0;
            }
        }
    }
}