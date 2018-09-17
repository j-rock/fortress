use control::GamepadId;

#[derive(Copy, Clone)]
pub enum ControllerId {
    Keyboard,
    Gamepad(GamepadId),
}
