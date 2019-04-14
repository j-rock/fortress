use crate::{
    control::GamepadId,
    dimensions::UpDownLeftRight,
};

#[derive(Copy, Clone)]
pub enum ControllerEvent {
    KeyboardUsed,
    GamepadConnected(GamepadId),
    GamepadDisconnected(GamepadId),
}

pub enum ControlEvent {
    PlayerMove(UpDownLeftRight),
    RedeployEntities,
}
