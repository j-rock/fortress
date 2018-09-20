use enum_iterator::IntoEnumIterator;
use file;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum Sound {
    Blast,
    Jump,
    Plop,
    Raygun,
    Slash,
}

impl Sound {
    pub fn to_path_buf(&self) -> PathBuf {
        let filename = match self {
            Sound::Blast => "blast.wav",
            Sound::Jump => "jump.wav",
            Sound::Plop => "plop.wav",
            Sound::Raygun => "raygun.wav",
            Sound::Slash => "slash.wav",
        };
        file::util::resource_path("audio", filename)
    }

    pub fn all_sounds() -> SoundEnumIterator {
        Self::into_enum_iter()
    }
}