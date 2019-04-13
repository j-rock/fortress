use crate::control::GamepadId;

#[derive(Copy, Clone)]
pub enum ControllerEvent {
    KeyboardUsed,
    GamepadConnected(GamepadId),
    GamepadDisconnected(GamepadId),
}

pub enum ControlEvent {
    RedeployEntities,
}
