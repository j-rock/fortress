use generational_slab::Key;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BulletId(Key);

impl BulletId {
    pub fn new(key: Key) -> BulletId {
        BulletId(key)
    }

    pub fn to_key(self) -> Key {
        self.0
    }
}

