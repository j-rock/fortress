use crate::{
    audio::AudioPlayer,
    dimensions::time::DeltaTime,
    players::{
        Player,
        PlayerId,
        PlayerSystem,
    },
};

pub struct WorldView<'a> {
    audio: &'a AudioPlayer,
    players: &'a mut PlayerSystem,
    dt: DeltaTime,
}

impl <'a> WorldView<'a> {
    pub fn new(audio: &'a AudioPlayer, players: &'a mut PlayerSystem, dt: DeltaTime) -> WorldView<'a> {
        WorldView {
            audio,
            players,
            dt,
        }
    }

    pub fn audio<'view>(&'view self) -> &AudioPlayer where 'view: 'a {
        self.audio
    }

    pub fn player_mut<'view>(&'view mut self, player_id: PlayerId) -> &'a mut Player where 'view: 'a {
        self.players.player_mut(player_id)
    }

    pub fn dt(&self) -> DeltaTime {
        self.dt
    }
}
