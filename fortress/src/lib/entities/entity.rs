use crate::{
    players::PlayerId,
    treasures::TreasureChestId,
    weapons::BulletId,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Entity {
    Bullet(PlayerId, BulletId),
    MapWall,
    Player,
    TreasureChest(TreasureChestId),
}
