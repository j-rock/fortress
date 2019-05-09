use crate::render::NamedSpriteSheet;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct SheetConfig {
    pub width: usize,
    pub height: usize,
    pub sprites: HashMap<String, SpriteConfig>,
}

#[derive(Copy, Clone, Deserialize)]
pub struct SpriteConfig {
    pub frame_width: usize,
    pub frame_height: usize,
}

#[derive(Clone, Deserialize)]
pub struct SpriteSheetConfig {
    pub render_background: bool,
    pub sheets: HashMap<NamedSpriteSheet, SheetConfig>
}
