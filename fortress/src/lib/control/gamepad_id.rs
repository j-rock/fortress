#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GamepadId {
    id: i32,
}

impl GamepadId {
    pub fn from_i32(id: i32) -> GamepadId {
        GamepadId {
            id
        }
    }
}

