use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum NamedSpriteSheet {
    SpriteSheet1
}

impl NamedSpriteSheet {
    pub fn to_lowercase_string(self) -> String {
        let mut string = format!("{:?}", self);
        string.make_ascii_lowercase();
        string
    }

    pub fn all_values() -> NamedSpriteSheetEnumIterator {
        Self::into_enum_iter()
    }
}
