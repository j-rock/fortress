use player::{
    Player,
    PlayerId,
};

pub struct PlayerSystem {
    player1: Option<Player>,
    player2: Option<Player>,
    player3: Option<Player>,
    player4: Option<Player>,
}

impl PlayerSystem {
    pub fn new() -> PlayerSystem {
        PlayerSystem {
            player1: None,
            player2: None,
            player3: None,
            player4: None
        }
    }

    pub fn enter_game(&mut self) -> Option<PlayerId> {
        if self.player1.is_none() {
            Some(PlayerId::Player1)
        } else if self.player2.is_none() {
            Some(PlayerId::Player2)
        } else if self.player3.is_none() {
            Some(PlayerId::Player3)
        } else if self.player4.is_none() {
            Some(PlayerId::Player4)
        } else {
            None
        }
    }
}