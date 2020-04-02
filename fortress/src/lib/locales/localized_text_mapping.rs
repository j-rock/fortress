use crate::locales::LocalizedTextKey;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct LocalizedTextMapping {
    mapping: HashMap<LocalizedTextKey, String>
}

impl LocalizedTextMapping {
    pub fn get(&self, key: &LocalizedTextKey) -> Option<&str> {
        self.mapping.get(key).map(|s| s.as_str())
    }
}
