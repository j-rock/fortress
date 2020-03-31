use crate::render::{
    Png,
    RawGlTexture,
    TextureStyle,
    TextureUnit,
};
use gl::{
    self,
    types::GLvoid,
};

pub struct PngTexture {
    raw_texture: RawGlTexture,
    texture_unit: TextureUnit,
    width: usize,
    height: usize,
}

impl PngTexture {
    pub fn new(png: Png, texture_style: TextureStyle, texture_unit: TextureUnit) -> PngTexture {
        let (width, height) = png.size();
        let png_vec = png.bytes();

        texture_unit.activate();
        let raw_texture = RawGlTexture::new();
        raw_texture.bind();
        texture_style.set_parameters();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::SRGB_ALPHA as i32,
                           width as i32, height as i32,
                           0, gl::RGBA, gl::UNSIGNED_BYTE, png_vec.as_ptr() as *const GLvoid);
        }
        raw_texture.unbind();

        PngTexture {
            raw_texture,
            texture_unit,
            width,
            height,
        }
    }

    pub fn activate(&self) -> TextureUnit {
        self.texture_unit.activate();
        self.raw_texture.bind();
        self.texture_unit
    }

    pub fn deactivate(&self) {
        self.raw_texture.unbind();
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
