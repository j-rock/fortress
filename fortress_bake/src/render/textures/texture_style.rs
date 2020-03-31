use gl;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TextureWrapMode {
    ClampToEdge, MirroredRepeat, Repeat
}

impl Default for TextureWrapMode {
    fn default() -> TextureWrapMode {
        TextureWrapMode::Repeat
    }
}

impl TextureWrapMode {
    pub fn raw(self) -> i32 {
        match self {
            TextureWrapMode::ClampToEdge => gl::CLAMP_TO_EDGE as i32,
            TextureWrapMode::MirroredRepeat => gl::MIRRORED_REPEAT as i32,
            TextureWrapMode::Repeat => gl::REPEAT as i32,
        }
    }
}

// Add more values as necessary.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TextureMinFilterMode {
    Nearest,
    Linear,
}

impl Default for TextureMinFilterMode {
    fn default() -> Self {
        Self::Nearest
    }
}

impl TextureMinFilterMode {
    pub fn raw(self) -> i32 {
        match self {
            Self::Nearest => gl::NEAREST as i32,
            Self::Linear => gl::LINEAR as i32,
        }
    }
}

// Add more values as necessary.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TextureMaxFilterMode {
    Nearest,
    Linear,
}

impl Default for TextureMaxFilterMode {
    fn default() -> Self {
        Self::Nearest
    }
}

impl TextureMaxFilterMode {
    pub fn raw(self) -> i32 {
        match self {
            Self::Nearest => gl::NEAREST as i32,
            Self::Linear => gl::LINEAR as i32,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct TextureStyle {
    pub wrap_s: TextureWrapMode,
    pub wrap_t: TextureWrapMode,
    pub min_filter: TextureMinFilterMode,
    pub max_filter: TextureMaxFilterMode,
}

impl TextureStyle {
    pub fn set_parameters(&self) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s.raw());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t.raw());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.min_filter.raw());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.max_filter.raw());
        }
    }
}

