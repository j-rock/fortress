use dimensions::LrDirection;

pub enum ControlEvent {
    PlayerJump,
    PlayerMove(LrDirection),
    PlayerRespawn,
    PlayerSlash,
}
