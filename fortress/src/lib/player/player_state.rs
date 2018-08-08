use player::{
    Player,
    PlayerBody,
    PlayerConfig,
};

pub struct PlayerState {
    pub config: PlayerConfig,
    pub body: PlayerBody,
}

impl PlayerState {
    pub fn new(config: PlayerConfig, body: PlayerBody) -> PlayerState {
        PlayerState {
            config,
            body,
        }
    }

    pub fn register(&mut self, player: *const Player) {
        self.body.register(player);
    }
}