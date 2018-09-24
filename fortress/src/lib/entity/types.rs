#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityType {
    BuffBox,
    Platform,

    CrossbowArrow(usize),

    Player,
    PlayerFootSensor,
    PlayerSwordSensor,

    Wraith,
}
