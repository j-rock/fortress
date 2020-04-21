use crate::players;

pub struct PlayerHudUpdate {
    data: [IndividualPlayerHudData; players::MAX_PLAYERS],
    len: usize,
}

impl PlayerHudUpdate {
    pub fn new() -> Self {
        PlayerHudUpdate {
            data: [IndividualPlayerHudData::default(); players::MAX_PLAYERS],
            len: 0,
        }
    }

    pub fn append(&mut self, update: IndividualPlayerHudData) {
        if self.len >= players::MAX_PLAYERS {
            return;
        }
        self.data[self.len] = update;
        self.len += 1;
    }

    pub fn get_first(&self) -> Option<&IndividualPlayerHudData> {
        if self.len == 0 {
            return None;
        }
        self.data.get(0)
    }
}

#[derive(Copy, Clone)]
pub struct IndividualPlayerHudData {
    pub skulls_collected: i64,
}

impl Default for IndividualPlayerHudData {
    fn default() -> Self {
        IndividualPlayerHudData {
            skulls_collected: 0
        }
    }
}
