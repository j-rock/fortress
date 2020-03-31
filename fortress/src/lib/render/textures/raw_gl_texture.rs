use gl::{
    self,
    types::GLuint,
};

pub struct RawGlTexture {
   texture_id: GLuint,
}

impl RawGlTexture {
    pub fn new() -> Self {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
        }
        RawGlTexture {
            texture_id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn texture_id(&self) -> GLuint {
        self.texture_id
    }
}

impl Drop for RawGlTexture {
    fn drop(&mut self) {
        unsafe {
            if self.texture_id != 0 {
                gl::DeleteTextures(1, &self.texture_id);
                self.texture_id = 0;
            }
        }
    }
}