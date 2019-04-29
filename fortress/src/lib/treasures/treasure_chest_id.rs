use generational_slab::Key;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TreasureChestId(Key);

impl TreasureChestId {
    pub fn new(key: Key) -> TreasureChestId {
        TreasureChestId(key)
    }
}