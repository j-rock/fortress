#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityType {
    CrossbowArrow(usize),

    Platform,

    Player,
    PlayerFootSensor,
    PlayerSwordSensor,

    Wraith,
}
