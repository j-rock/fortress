use crate::dimensions::LrDirection;
use nalgebra::Vector2;

#[derive(Copy, Clone)]
pub enum OctoDirection {
    Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight
}

impl OctoDirection {
    pub fn to_direction(self) -> Vector2<f64> {
        let unnormalized = match self {
            OctoDirection::Up => Vector2::new(0.0, 1.0),
            OctoDirection::Down => Vector2::new(0.0, -1.0),
            OctoDirection::Left => Vector2::new(-1.0, 0.0),
            OctoDirection::Right => Vector2::new(1.0, 0.0),
            OctoDirection::UpLeft => Vector2::new(-1.0, 1.0),
            OctoDirection::UpRight => Vector2::new(1.0, 1.0),
            OctoDirection::DownLeft => Vector2::new(-1.0, -1.0),
            OctoDirection::DownRight => Vector2::new(1.0, -1.0),
        };
        unnormalized.normalize()
    }

    pub fn to_lr_direction(self) -> Option<LrDirection> {
        match self {
            OctoDirection::Left | OctoDirection::UpLeft | OctoDirection::DownLeft => Some(LrDirection::Left),
            OctoDirection::Right | OctoDirection::UpRight | OctoDirection::DownRight => Some(LrDirection::Right),
            _ => None,
        }
    }

    pub fn from(up: bool, down: bool, left: bool, right: bool) -> Option<OctoDirection> {
        if up {
            if left {
                Some(OctoDirection::UpLeft)
            } else if right {
                Some(OctoDirection::UpRight)
            } else {
                Some(OctoDirection::Up)
            }
        } else if down {
            if left {
                Some(OctoDirection::DownLeft)
            } else if right {
                Some(OctoDirection::DownRight)
            } else {
                Some(OctoDirection::Down)
            }
        } else {
            if left {
                Some(OctoDirection::Left)
            } else if right {
                Some(OctoDirection::Right)
            } else {
                None
            }
        }
    }
}
