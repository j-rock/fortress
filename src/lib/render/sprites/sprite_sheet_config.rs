use crate::render::NamedSpriteSheet;
use std::collections::HashMap;

#[derive(Copy, Clone, Deserialize)]
pub struct SheetData {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Deserialize)]
pub struct SpriteSheetConfig {
    pub sheets: HashMap<NamedSpriteSheet, SheetData>
}
