use crate::{
    app::StatusOr,
    render::{
        FrameBuffer,
        FrameBufferTexture,
    }
};
use gl;
use glm;

pub struct BloomPingPongBuffer {
    frame_buffer: FrameBuffer,
    color_texture: FrameBufferTexture,
}

impl BloomPingPongBuffer {
    pub fn new(screen_size: glm::IVec2) -> StatusOr<Self> {
        let frame_buffer = FrameBuffer::new();
        frame_buffer.bind();
        let color_texture = FrameBufferTexture::new(screen_size, gl::COLOR_ATTACHMENT0);
        FrameBuffer::check_complete()?;
        Ok(BloomPingPongBuffer {
            frame_buffer,
            color_texture
        })
    }

    pub fn bind_frame_buffer(&self) {
        self.frame_buffer.bind();
    }

    pub fn bind_color_texture(&self) {
        self.color_texture.bind();
    }


}
