#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Entity {
    BuffBox,
    BuffDrop,
    Wall,
    CrossbowArrow(usize),
    Player,
    PlayerSwordSensor,
    Wraith,
}
