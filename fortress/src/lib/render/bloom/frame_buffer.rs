use crate::app::StatusOr;
use gl::{
    self,
    types::GLuint,
};

pub struct FrameBuffer {
    frame_buffer: GLuint,
}

impl FrameBuffer {
    pub fn new() -> Self {
        let mut frame_buffer: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut frame_buffer);
        }
        FrameBuffer {
            frame_buffer
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn check_complete() -> StatusOr<()> {
        unsafe {
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err(String::from("Framebuffer not complete!"));
            }
        }
        Ok(())
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            if self.frame_buffer != 0 {
                gl::DeleteFramebuffers(1, &self.frame_buffer);
                self.frame_buffer = 0;
            }
        }
    }
}
