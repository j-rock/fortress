use crate::file;
use enum_iterator::Sequence;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Sequence)]
pub enum Sound {
    BarrelDestroy,
    BarrelHit,
    CollectItem,
    EnemyGeneratorHurt,
    EnemyGeneratorKilled,
    EnemyKilled,
    HeroSwitch,
    JoinGame,
    ShootSingleFireball,
    ShootSpecial,
}

impl Sound {
    pub fn to_path_buf(self) -> PathBuf {
        let filename = match self {
            Sound::BarrelDestroy => "barrel_destroy.wav",
            Sound::BarrelHit => "barrel_hit.wav",
            Sound::CollectItem => "collect_item.wav",
            Sound::EnemyGeneratorHurt => "enemy_generator_hurt.wav",
            Sound::EnemyGeneratorKilled => "enemy_generator_killed.wav",
            Sound::EnemyKilled => "enemy_killed.wav",
            Sound::HeroSwitch => "hero_switch.wav",
            Sound::JoinGame => "join_game.wav",
            Sound::ShootSingleFireball => "shoot_single_fireball.wav",
            Sound::ShootSpecial => "shoot_special.wav",
        };
        file::util::resource_path("audio", filename)
    }

    pub fn all_sounds() -> impl Iterator<Item = Self> {
        enum_iterator::all::<Self>()
    }
}