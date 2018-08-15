use dimensions::LrDirection;

pub enum ControlEvent {
    PlayerJump,
    PlayerMove(LrDirection),
    PlayerSlash,

    RespawnEntities,
}
