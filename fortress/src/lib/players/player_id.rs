use players;

#[derive(Copy, Clone)]
pub struct PlayerId {
    id: usize
}

impl PlayerId {
    pub fn from_usize(id: usize) -> Option<PlayerId> {
        if id < players::MAX_PLAYERS {
            Some(PlayerId {
                id
            })
        } else {
            None
        }
    }

    pub fn as_usize(self) -> usize {
        self.id
    }
}