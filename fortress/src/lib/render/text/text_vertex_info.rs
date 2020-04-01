use glm;
use glyph_brush::GlyphVertex;

pub struct TextVertexInfo {
    _screen_top_left: glm::Vec2,
    _screen_bottom_right: glm::Vec2,
    _screen_z_pos: f32,
    _texel_top_left: glm::Vec2,
    _texel_bottom_right: glm::Vec2,
    _color: glm::Vec4,
}

impl TextVertexInfo {
    pub fn from(_vertex: GlyphVertex) -> Self {
        TextVertexInfo {
            _screen_top_left: glm::vec2(0.0, 0.0),
            _screen_bottom_right: glm::vec2(0.0, 0.0),
            _screen_z_pos: 0.0,
            _texel_top_left: glm::vec2(0.0, 0.0),
            _texel_bottom_right: glm::vec2(0.0, 0.0),
            _color: glm::vec4(0.0, 0.0, 0.0, 0.0),
        }
    }
}

