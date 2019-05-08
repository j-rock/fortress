use generational_slab::Key;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ItemId(Key);

impl ItemId {
    pub fn from_key(key: Key) -> ItemId {
        ItemId(key)
    }

    pub fn key(self) -> Key {
        self.0
    }
}
