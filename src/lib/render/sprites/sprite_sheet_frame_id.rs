use crate::render::NamedSpriteSheet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SpriteSheetFrameId {
    pub name: String,
    pub sprite_sheet: NamedSpriteSheet,
}
