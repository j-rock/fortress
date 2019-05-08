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

    pub fn from_radians(angle: f64) -> LrDirection {
        if angle.cos() <= 0.0 {
            LrDirection::Left
        } else {
            LrDirection::Right
        }
    }
}