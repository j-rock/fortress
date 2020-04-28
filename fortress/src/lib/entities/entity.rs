use crate::{
    enemies::{
        EnemyGeneratorId,
        EnemyId,
    },
    items::{
        barrels::BarrelId,
        ItemId,
    },
    players::PlayerId,
    weapons::BulletId,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Entity {
    Barrel(BarrelId),
    Bullet(PlayerId, BulletId),
    Enemy(EnemyId),
    EnemyGenerator(EnemyGeneratorId),
    Item(ItemId),
    MapWall,
    Player(PlayerId),
}
