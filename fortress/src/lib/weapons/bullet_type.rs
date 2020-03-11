#[derive(Copy, Clone)]
pub enum BulletType {
    Normal,
    Special
}

impl BulletType {
    pub fn is_special(self) -> bool {
        match self {
            Self::Special => true,
            _ => false,
        }
    }
}

