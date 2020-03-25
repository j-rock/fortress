use crate::render::NamedSpriteSheet;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SpriteSheetFrameId {
    pub name: String,
    pub sprite_sheet: NamedSpriteSheet,
}
