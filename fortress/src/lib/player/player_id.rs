pub enum PlayerId {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl PlayerId {
    pub fn from(i: usize) -> Option<PlayerId> {
        match i {
            0 => Some(PlayerId::Player1),
            1 => Some(PlayerId::Player2),
            2 => Some(PlayerId::Player3),
            3 => Some(PlayerId::Player4),
            _ => None
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            PlayerId::Player1 => 0,
            PlayerId::Player2 => 1,
            PlayerId::Player3 => 2,
            PlayerId::Player4 => 3
        }
    }
}