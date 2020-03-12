use crate::file;
use enum_iterator::IntoEnumIterator;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum Sound {
    CollectItem,
    EnemyKilled,
    HeroSwitch,
    JoinGame,
    ShootSingleFireball,
}

impl Sound {
    pub fn to_path_buf(self) -> PathBuf {
        let filename = match self {
            Sound::CollectItem => "collect_item.wav",
            Sound::EnemyKilled => "enemy_killed.wav",
            Sound::HeroSwitch => "hero_switch.wav",
            Sound::JoinGame => "join_game.wav",
            Sound::ShootSingleFireball => "shoot_single_fireball.wav",
        };
        file::util::resource_path("audio", filename)
    }

    pub fn all_sounds() -> <Self as IntoEnumIterator>::Iterator {
        Self::into_enum_iter()
    }
}