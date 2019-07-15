use crate::{
    app::StatusOr,
    audio::{
        AudioConfig,
        Sound
    },
    file::{
        ConfigWatcher,
        SimpleConfigManager
    },
};
use hashbrown::HashMap;
use sdl2::mixer::{
    Chunk,
    self,
};

const CHUNK_SIZE: i32 = 2048;

pub struct AudioPlayer {
    config: SimpleConfigManager<AudioConfig>,
    chunks: HashMap<Sound, Chunk>
}

impl AudioPlayer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<AudioPlayer> {
        mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, CHUNK_SIZE)
            .map_err(|e| format!("Error opening audio: {}", e))?;

        let sound_chunks: StatusOr<HashMap<Sound, Chunk>> = Sound::all_sounds()
            .map(|sound| {
                let path_buf = sound.to_path_buf();
                Chunk::from_file(path_buf).map(|chunk| (sound, chunk))
            })
            .collect();

        let config = SimpleConfigManager::from_config_resource(config_watcher, "audio.conf")?;
        Self::set_volume(config.get());

        Ok(AudioPlayer {
            config,
            chunks: sound_chunks?
        })
    }

    pub fn update(&mut self) {
        if self.config.update() {
            Self::set_volume(self.config.get());
        }
    }

    pub fn play_sound(&self, sound: Sound) {
        if let Some(chunk) = self.chunks.get(&sound) {
            let channel = mixer::Channel::all();
            match channel.play(chunk, 0) {
                _ => {}
            }
        }
    }

    fn set_volume(config: &AudioConfig) {
        let channel = mixer::Channel::all();
        channel.set_volume((config.sound_volume * mixer::MAX_VOLUME as f64) as i32);
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        mixer::close_audio();
    }
}