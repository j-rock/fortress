#[derive(Copy, Clone)]
pub enum LrDirection {
    Left, Right
}

impl LrDirection {
    pub fn is_left(self) -> bool {
        match self {
            LrDirection::Left => true,
            _ => false,
        }
    }
}