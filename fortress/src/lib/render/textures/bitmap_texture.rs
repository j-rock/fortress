use crate::render::{
    RawGlTexture,
    SerializableBitmap,
    TextureStyle,
    TextureUnit,
};
use gl::{
    self,
    types::GLvoid,
};

pub struct BitmapTexture {
    raw_texture: RawGlTexture,
    texture_unit: TextureUnit,
}

impl BitmapTexture {
    pub fn new(image: SerializableBitmap, texture_style: TextureStyle, texture_unit: TextureUnit) -> Self {
        let (width, height) = image.size();
        let image_bytes = image.image_bytes();

        texture_unit.activate();
        let raw_texture = RawGlTexture::new();
        raw_texture.bind();
        texture_style.set_parameters();
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32,
                           width as i32, height as i32,
                           0, gl::RED, gl::UNSIGNED_BYTE, image_bytes.as_ptr() as *const GLvoid);
        }
        raw_texture.unbind();

        BitmapTexture {
            raw_texture,
            texture_unit,
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
}