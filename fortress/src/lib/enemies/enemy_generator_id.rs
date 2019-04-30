use generational_slab::Key;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnemyGeneratorId(Key);

impl EnemyGeneratorId {
    pub fn from_key(key: Key) -> EnemyGeneratorId {
        EnemyGeneratorId(key)
    }

    pub fn key(self) -> Key {
        self.0
    }
}
