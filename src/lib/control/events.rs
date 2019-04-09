use crate::{
    control::GamepadId,
    dimensions::LrDirection
};

#[derive(Copy, Clone)]
pub enum ControllerEvent {
    KeyboardUsed,
    GamepadConnected(GamepadId),
    GamepadDisconnected(GamepadId),
}

pub enum ControlEvent {
    PlayerFire,
    PlayerJump,
    PlayerMove(LrDirection),
    PlayerSlash,

    RedeployEntities,
}