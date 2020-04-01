use gl;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureUnit {
    Texture0,
    Texture1,
    Texture2,
}

impl TextureUnit {
    pub fn activate(&self) {
        let active_texture = match self {
            Self::Texture0 => gl::TEXTURE0,
            Self::Texture1 => gl::TEXTURE1,
            Self::Texture2 => gl::TEXTURE2,
        };
        unsafe {
            gl::ActiveTexture(active_texture);
        }
    }

    pub fn to_texture_uniform(self) -> i32 {
        match self {
            Self::Texture0 => 0,
            Self::Texture1 => 1,
            Self::Texture2 => 2,
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
