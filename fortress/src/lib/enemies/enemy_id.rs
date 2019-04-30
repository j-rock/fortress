use generational_slab::Key;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnemyId(Key);

impl EnemyId {
    pub fn from_key(key: Key) -> EnemyId {
        EnemyId(key)
    }

    pub fn key(self) -> Key {
        self.0
    }
}