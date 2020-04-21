use crate::items::{
    ItemConfig,
    types::SkullType,
};

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

    pub fn add_to_count(&mut self, config: &ItemConfig, skull: SkullType) {
        let skull_value = config.skull_value.get(&skull).unwrap_or(&0);
        self.current_count += *skull_value;
    }
}