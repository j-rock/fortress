use enum_iterator::IntoEnumIterator;
use serde_json;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum NamedSpriteSheet {
    SpriteSheet1,
    GalaxyGround,
    Heroes,
}

impl NamedSpriteSheet {
    pub fn to_directory_basename(self) -> String {
        format!("{:?}", self)
    }

    pub fn all_values() -> impl Iterator<Item=NamedSpriteSheet> {
        Self::into_enum_iter()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        serde_json::de::from_str(s).ok()
    }
}
