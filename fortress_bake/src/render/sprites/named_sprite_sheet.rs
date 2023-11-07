use enum_iterator::Sequence;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Sequence)]
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
        enum_iterator::all::<Self>()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        ron::de::from_str(s).ok()
    }
}
