use gl::{
    self,
    types::GLuint,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureUnit {
    Texture0,
    Texture1,
    Texture2,
}

impl TextureUnit {
    pub fn activate(&self) {
        unsafe {
            gl::ActiveTexture(self.to_gluint());
        }
    }

    pub fn to_gluint(self) -> GLuint {
        match self {
            Self::Texture0 => gl::TEXTURE0,
            Self::Texture1 => gl::TEXTURE1,
            Self::Texture2 => gl::TEXTURE2,
        }
    }

    pub fn uniform_name(&self) -> &'static str {
        match self {
            Self::Texture0 => "texture0",
            Self::Texture1 => "texture1",
            Self::Texture2 => "texture2",
        }
    }
}
