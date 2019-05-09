use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum NamedSpriteSheet {
    SpriteSheet1,
    GalaxyGround,
}

impl NamedSpriteSheet {
    pub fn to_lowercase_string(self) -> String {
        format!("{:?}", self)
    }

    pub fn all_values(render_background: bool) -> Vec<NamedSpriteSheet> {
        if render_background {
            Self::into_enum_iter().collect()
        } else {
            Self::into_enum_iter().filter(|value| *value != NamedSpriteSheet::GalaxyGround).collect()
        }
    }
}
