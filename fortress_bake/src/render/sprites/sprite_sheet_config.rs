use crate::render::{
    NamedSpriteSheet,
    TextureStyle,
};
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct SheetConfig {
    pub width: usize,
    pub height: usize,
    pub style: TextureStyle,
    pub sprites: HashMap<String, SpriteConfig>,
}

#[derive(Clone, Deserialize)]
pub struct SpriteConfig {
    pub frame_width: usize,
    pub frame_height: usize,
}

#[derive(Clone, Deserialize)]
pub struct SpriteSheetConfig {
    pub sheets: HashMap<NamedSpriteSheet, SheetConfig>
}
