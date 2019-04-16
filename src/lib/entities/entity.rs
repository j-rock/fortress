use crate::{
    players::PlayerId,
    weapons::BulletId,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Entity {
    BuffBox,
    BuffDrop,
    MapWall,
    Bullet(PlayerId, BulletId),
    Player,
    Wraith,
}
