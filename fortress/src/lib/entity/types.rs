#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityType {
    BuffBox,
    BuffDrop,
    Platform,

    CrossbowArrow(usize),

    Player,
    PlayerFootSensor,
    PlayerSwordSensor,

    Wraith,
}
