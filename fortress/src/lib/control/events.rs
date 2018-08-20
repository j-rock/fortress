use dimensions::LrDirection;

pub enum ControlEvent {
    PlayerFire,
    PlayerJump,
    PlayerMove(LrDirection),
    PlayerSlash,

    RespawnEntities,
}
