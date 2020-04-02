use crate::render::{
    RawGlTexture,
    TextureStyle,
    TextureWrapMode,
    TextureMinFilterMode,
    TextureMaxFilterMode,
};
use gl::{
    self,
    types::GLint,
};
use glm;

pub struct GlyphTexture {
    raw: RawGlTexture,
}

impl GlyphTexture {
    pub fn new(size: glm::IVec2) -> Self {
        let raw = RawGlTexture::new();
        raw.bind();
        let texture_style = TextureStyle {
            wrap_s: TextureWrapMode::ClampToEdge,
            wrap_t: TextureWrapMode::ClampToEdge,
            min_filter: TextureMinFilterMode::Linear,
            max_filter: TextureMaxFilterMode::Linear,
        };
        texture_style.set_parameters();

        unsafe {
            let mut original_unpack_alignment: GLint = 0;
            gl::GetIntegerv(gl::UNPACK_ALIGNMENT, &original_unpack_alignment);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED, size.x as i32, height as i32, 0, gl::RED, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, original_unpack_alignment);
        }

        GlyphTexture {
            raw
        }
    }
}

