use crate::render::{
    RawGlTexture,
    TextureMinFilterMode,
    TextureMaxFilterMode,
    TextureStyle,
    TextureWrapMode,
};
use gl::{
    self,
    types::{
        GLenum,
        GLsizei,
    },
};
use glm;

pub struct FrameBufferTexture {
    raw_texture: RawGlTexture,
}

impl FrameBufferTexture {
    pub fn new(screen_size: glm::IVec2, attachment: GLenum) -> Self {
        let raw_texture = RawGlTexture::new();
        raw_texture.bind();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, screen_size.x as GLsizei, screen_size.y as GLsizei, 0,
                           gl::RGBA, gl::FLOAT, std::ptr::null());
        }

        let texture_style = TextureStyle {
            wrap_s: TextureWrapMode::ClampToEdge,
            wrap_t: TextureWrapMode::ClampToEdge,
            min_filter: TextureMinFilterMode::Linear,
            max_filter: TextureMaxFilterMode::Linear,
        };
        texture_style.set_parameters();

        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment, gl::TEXTURE_2D, raw_texture.texture_id(), 0);
        }

        FrameBufferTexture {
            raw_texture
        }
    }

    pub fn bind(&self) {
        self.raw_texture.bind();
    }
}
