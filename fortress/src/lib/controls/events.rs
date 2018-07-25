use dimensions::LrDirection;

pub enum ControlEvent {
    PlayerMove(LrDirection),
    PlayerRespawn,
}
