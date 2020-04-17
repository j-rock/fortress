use crate::{
    app::StatusOr,
    render::{
        BloomConfig,
        BloomCompositorShader,
        BloomPingPongBuffer,
        BlurShader,
        DepthRenderBuffer,
        FrameBuffer,
        FrameBufferTexture,
        TextureUnit,
    },
};
use gl::{
    self,
    types::GLenum,
};
use glm;

pub struct BloomPipeline {
    blur_shader: BlurShader,
    compositor_shader: BloomCompositorShader,
    scene_buffer: FrameBuffer,
    color_texture: FrameBufferTexture,
    bloom_texture: FrameBufferTexture,
    ping_pong0: BloomPingPongBuffer,
    ping_pong1: BloomPingPongBuffer,
    _depth_render_buffer: DepthRenderBuffer,
}

impl BloomPipeline {
    pub fn new(screen_size: glm::IVec2) -> StatusOr<Self> {
        let blur_shader = BlurShader::new()?;
        let compositor_shader = BloomCompositorShader::new()?;

        let scene_buffer = FrameBuffer::new();
        scene_buffer.bind();

        let color_texture = FrameBufferTexture::new(screen_size, gl::COLOR_ATTACHMENT0);
        let bloom_texture = FrameBufferTexture::new(screen_size, gl::COLOR_ATTACHMENT1);
        let depth_render_buffer = DepthRenderBuffer::new(screen_size);

        let attachments: [GLenum; 2] = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1];
        unsafe {
            gl::DrawBuffers(attachments.len() as i32, attachments.as_ptr());
        }
        FrameBuffer::check_complete()?;
        scene_buffer.unbind();

        let ping_pong0 = BloomPingPongBuffer::new(screen_size)?;
        let ping_pong1 = BloomPingPongBuffer::new(screen_size)?;

        Ok(BloomPipeline {
            blur_shader,
            compositor_shader,
            scene_buffer,
            color_texture,
            bloom_texture,
            ping_pong0,
            ping_pong1,
            _depth_render_buffer: depth_render_buffer,
        })
    }

    pub fn begin(&self) {
        self.scene_buffer.bind();
        Self::clear();
    }

    pub fn blur(&mut self, config: &BloomConfig) {
        self.scene_buffer.unbind();
        self.blur_pass(config);
        self.draw_to_default_framebuffer(config);
    }

    fn blur_pass(&mut self, config: &BloomConfig) {
        self.blur_shader.activate();

        let num_iterations = 2 * config.num_passes;
        for i in 0..num_iterations {
            let horizontal = i % 2 == 0;
            if horizontal {
                self.ping_pong1.bind_frame_buffer();
            } else {
                self.ping_pong0.bind_frame_buffer();
            }
            self.blur_shader.set_horizontal_mode(horizontal);
            TextureUnit::Texture0.activate();
            match horizontal {
                _ if i == 0 => self.bloom_texture.bind(),
                true => self.ping_pong0.bind_color_texture(),
                false => self.ping_pong1.bind_color_texture()
            };
            self.blur_shader.draw();
        }
    }

    fn draw_to_default_framebuffer(&mut self, config: &BloomConfig) {
        self.scene_buffer.unbind();
        Self::clear();
        self.compositor_shader.draw(config, &self.color_texture, &self.ping_pong0);
    }

    fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
