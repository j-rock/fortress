use crate::players;
use generational_slab::Key;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayerId(Key);

impl PlayerId {
    pub fn from_key(key: Key) -> Option<PlayerId> {
        if key.to_raw() < players::MAX_PLAYERS {
            Some(PlayerId(key))
        } else {
            None
        }
    }

    pub fn to_key(self) -> Key {
        self.0
    }

    pub fn to_raw_usize(self) -> usize {
        self.0.to_raw()
    }
}