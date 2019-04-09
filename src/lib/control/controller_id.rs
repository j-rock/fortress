use crate::control::GamepadId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ControllerId {
    Keyboard,
    Gamepad(GamepadId),
}
