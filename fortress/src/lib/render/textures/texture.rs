use crate::{
    image::Png,
    render::ShaderProgram,
};
use gl::{
    self,
    types::*
};

#[derive(Copy, Clone)]
pub struct TextureId(GLuint);
#[derive(Copy, Clone)]
pub struct TextureUnit(usize);

impl TextureUnit {
    pub fn to_gluint(self) -> GLuint {
        match self.0 {
            0 => gl::TEXTURE0,
            1 => gl::TEXTURE1,
            2 => gl::TEXTURE2,
            _ => panic!("Unimplemented texture unit sampler.")
        }
    }

    pub fn uniform_name(&self) -> &'static str {
       match self.0 {
           0 => "texture0",
           1 => "texture1",
           2 => "texture2",
           _ => panic!("Unimplemented texture unit sampler.")
       }
    }
}

pub struct Texture {
    texture_id: TextureId,
    texture_unit: TextureUnit,
    _png_vec: Vec<u8>,
}

impl Texture {
    pub fn new(png: Png, texture_unit: usize) -> Texture {
        let (width, height) = png.size();
        let png_vec = png.flattened_copy_bytes();

        let mut texture_id = 0;
        let texture_unit = TextureUnit(texture_unit);

        unsafe {
            gl::ActiveTexture(texture_unit.to_gluint());

            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::SRGB_ALPHA as i32,
                           width as i32, height as i32,
                           0, gl::RGBA, gl::UNSIGNED_BYTE, png_vec.as_ptr() as *const GLvoid);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            texture_id: TextureId(texture_id),
            texture_unit,
            _png_vec: png_vec,
        }
    }

    pub fn activate(&self, shader_program: &mut ShaderProgram) {
        let raw_texture_unit = self.texture_unit.to_gluint();
        unsafe {
            gl::ActiveTexture(raw_texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id.0);
        }

        shader_program.set_gluint(self.texture_unit.uniform_name(), raw_texture_unit);
    }

    pub fn deactivate(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn texture_id(&self) -> TextureId {
        self.texture_id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.texture_id.0 != 0 {
            unsafe {
                gl::DeleteTextures(1, &self.texture_id.0);
            }
        }
    }
}