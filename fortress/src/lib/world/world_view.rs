use crate::{
    audio::AudioPlayer,
    dimensions::time::DeltaTime,
    enemies::EnemySystem,
    items::ItemSystem,
    particles::ParticleSystem,
    players::PlayerSystem,
};

pub struct WorldView<'a> {
    pub audio: &'a AudioPlayer,
    pub players: &'a mut PlayerSystem,
    pub enemies: &'a mut EnemySystem,
    pub items: &'a mut ItemSystem,
    pub particles: &'a mut ParticleSystem,
    pub dt: DeltaTime,
}

