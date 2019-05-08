use crate::{
    enemies::{
        EnemyGeneratorId,
        EnemyId,
    },
    items::ItemId,
    players::PlayerId,
    weapons::BulletId,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Entity {
    BuffBox,
    BuffDrop,
    Bullet(PlayerId, BulletId),
    Enemy(EnemyId),
    EnemyGenerator(EnemyGeneratorId),
    Item(ItemId),
    MapWall,
    Player(PlayerId),
}
