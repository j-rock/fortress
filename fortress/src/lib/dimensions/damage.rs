#[derive(Copy, Clone)]
pub struct Damage(i64);

impl Damage {
    pub fn new(val: i64) -> Damage {
        Damage(val)
    }

    pub fn value(self) -> i64 {
        self.0
    }
}