use crate::{
    audio::AudioPlayer,
    dimensions::time::DeltaTime,
    players::PlayerSystem,
    treasures::TreasureSystem,
};

pub struct WorldView<'a> {
    pub audio: &'a AudioPlayer,
    pub players: &'a mut PlayerSystem,
    pub treasures: &'a mut TreasureSystem,
    pub dt: DeltaTime,
}

