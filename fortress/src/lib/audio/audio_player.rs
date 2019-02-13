use crate::{
    app::StatusOr,
    audio::Sound,
};
use sdl2::mixer::{
    Chunk,
    self,
};
use std::collections::HashMap;

const CHUNK_SIZE: i32 = 512;

pub struct AudioPlayer {
    chunks: HashMap<Sound, Chunk>
}

impl AudioPlayer {
    pub fn new() -> StatusOr<AudioPlayer> {
        mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, CHUNK_SIZE)
            .unwrap_or_else(|e| println!("Error opening audio: {}", e));

        let sound_chunks: StatusOr<HashMap<Sound, Chunk>> = Sound::all_sounds()
            .map(|sound| {
                let path_buf = sound.to_path_buf();
                Chunk::from_file(path_buf).map(|chunk| (sound, chunk))
            })
            .collect();

        Ok(AudioPlayer {
            chunks: sound_chunks?
        })
    }

    pub fn play_sound(&self, sound: Sound) {
        if let Some(chunk) = self.chunks.get(&sound) {
            let channel = mixer::Channel::all();
            match channel.play(chunk, 0) {
                _ => {}
            }
        }
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        mixer::close_audio();
    }
}