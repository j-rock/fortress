#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GamepadId {
    id: u32,
}

impl GamepadId {
    pub fn from_u32(id: u32) -> GamepadId {
        GamepadId {
            id
        }
    }
}

