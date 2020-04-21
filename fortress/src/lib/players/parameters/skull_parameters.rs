use crate::items::types::SkullType;

pub struct SkullParameters {
    current_count: i64,
}

impl Default for SkullParameters {
    fn default() -> Self {
        SkullParameters {
            current_count: 0,
        }
    }
}

impl SkullParameters {
    pub fn current_count(&self) -> i64 {
        self.current_count
    }

    pub fn add_to_count(&mut self, skull: SkullType) {
        self.current_count += match skull {
            SkullType::Mega => 5,
            SkullType::Regular => 1,
        };
    }
}