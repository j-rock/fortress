#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Entity {
    BuffBox,
    BuffDrop,
    MapWall,
    CrossbowArrow(usize),
    Player,
    PlayerSwordSensor,
    Wraith,
}
