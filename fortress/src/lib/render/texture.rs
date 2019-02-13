use crate::{
    dimensions::Pixels,
    image::Png
};
use gl::{
    self,
    types::*
};

pub struct Texture {
    pub texture_id: GLuint,
    pub width: Pixels,
    pub height: Pixels,
}

impl Texture {
    pub fn new(png: &Png) -> Texture {
        let mut texture = Texture {
            texture_id: 0,
            width: 0,
            height: 0,
        };
        texture.bind(png);
        texture
    }

    fn bind(&mut self, png: &Png) {
        let (width, height) = png.size();
        self.width = width;
        self.height = height;
        let png_vec = png.flattened_copy_bytes();
        unsafe {
            if self.texture_id == 0 {
                gl::GenTextures(1, &mut self.texture_id);
            }
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::SRGB_ALPHA as i32,
                           self.width as i32, self.height as i32,
                           0, gl::RGBA, gl::UNSIGNED_BYTE, png_vec.as_ptr() as *const GLvoid);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.texture_id != 0 {
            unsafe {
                gl::DeleteTextures(1, &self.texture_id);
            }
        }
    }
}
