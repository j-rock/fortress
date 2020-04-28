use generational_slab::Key;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BarrelId(Key);

impl BarrelId {
    pub fn from_key(key: Key) -> Self {
        BarrelId(key)
    }

    pub fn key(self) -> Key {
        self.0
    }
}
