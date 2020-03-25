#[derive(Copy, Clone)]
pub struct Reverse {
    pub horizontally: bool,
    pub vertically: bool,
}

impl Reverse {
    pub fn none() -> Reverse {
        Reverse {
            horizontally: false,
            vertically: false,
        }
    }

    pub fn horizontally() -> Reverse {
        Reverse {
            horizontally: true,
            vertically: false,
        }
    }

    pub fn vertically() -> Reverse {
        Reverse {
            horizontally: false,
            vertically: true,
        }
    }
}