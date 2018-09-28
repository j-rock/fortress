#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityType {
    BuffBox,
    BuffDrop,

    Wall,

    CrossbowArrow(usize),

    Player,
    PlayerFootSensor,
    PlayerSwordSensor,

    Wraith,
}
