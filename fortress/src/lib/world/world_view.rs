use crate::{
    audio::AudioPlayer,
    dimensions::time::DeltaTime,
    players::PlayerSystem,
};

pub struct WorldView<'a> {
    pub audio: &'a AudioPlayer,
    pub players: &'a mut PlayerSystem,
    pub dt: DeltaTime,
}

