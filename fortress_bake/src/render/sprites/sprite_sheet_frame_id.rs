use crate::render::NamedSpriteSheet;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SpriteSheetFrameId {
    name: String,
    sprite_sheet: NamedSpriteSheet,
}

impl SpriteSheetFrameId {
    pub fn new(name: String, sprite_sheet: NamedSpriteSheet) -> Self {
        SpriteSheetFrameId {
            name,
            sprite_sheet,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn sprite_sheet(&self) -> NamedSpriteSheet {
        self.sprite_sheet
    }
}
