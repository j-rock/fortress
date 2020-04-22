#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq)]
pub enum Criticality {
    Normal,
    Crit,
}


#[derive(Copy, Clone)]
pub struct Damage {
    value: i64,
    criticality: Criticality,
}

impl Damage {
    pub fn new(value: i64, criticality: Criticality) -> Self {
        Damage {
            value,
            criticality,
        }
    }

    pub fn value(self) -> i64 {
        self.value
    }

    pub fn criticality(self) -> Criticality {
        self.criticality
    }
}