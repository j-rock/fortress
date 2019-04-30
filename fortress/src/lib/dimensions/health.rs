use crate::dimensions::Damage;

#[derive(Copy, Clone, Debug)]
pub struct Health {
   amount: i64,
}

impl Health {
    pub fn new(amount: i64) -> Health {
        Health {
            amount
        }
    }

    pub fn alive(&self) -> bool {
        self.amount > 0
    }

    pub fn withdraw(&mut self, damage: Damage) {
        self.amount -= damage.value();
    }
}

