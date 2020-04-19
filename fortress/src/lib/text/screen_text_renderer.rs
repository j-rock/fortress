use crate::{
    app::StatusOr,
    file,
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        BitmapTexture,
        ShaderProgram,
        ShaderUniformKey,
        Texel,
        TextureUnit,
    },
    text::{
        Base10CharIterator,
        GlyphId,
        Locale,
        TextContent,
        ScreenTextRequest,
        TextResolver,
    },
};
use gl::{
    self,
    types::GLsizei,
};
use glm;
use std::ffi::CString;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    FontTexture,
    ScreenWindowSize,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let string = match self {
            Self::FontTexture => "font",
            Self::ScreenWindowSize => "screen_window_size",
        };
        CString::new(string).expect("Bad cstring")
    }
}

pub struct ScreenTextRenderer {
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<PositionAttr>,
    attr_glyph_size: Attribute<GlyphSizeAttr>,
    attr_texel: Attribute<TexelAttr>,
    attr_color: Attribute<ColorAttr>,

    screen_size: glm::Vec2,
}

impl ScreenTextRenderer {
    pub fn new() -> StatusOr<Self> {
        let vertex = file::util::resource_path("shaders", "screen_text_vert.glsl");
        let geometry = file::util::resource_path("shaders", "screen_text_geo.glsl");
        let fragment = file::util::resource_path("shaders", "text_frag.glsl");
        let mut shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        shader_program.activate();
        shader_program.set_texture(UniformKey::FontTexture, TextureUnit::Texture0);

        let mut attribute_program_builder = AttributeProgram::builder();
        let attr_pos = attribute_program_builder.add_attribute();
        let attr_glyph_size = attribute_program_builder.add_attribute();
        let attr_texel = attribute_program_builder.add_attribute();
        let attr_color = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(ScreenTextRenderer {
            shader_program,
            attribute_program,
            attr_pos,
            attr_glyph_size,
            attr_texel,
            attr_color,
            screen_size: glm::vec2(0.0, 0.0),
        })
    }

    pub fn set_screen_size(&mut self, screen_size: glm::IVec2) {
        self.screen_size = glm::vec2(screen_size.x as f32, screen_size.y as f32);
    }

    pub fn queue(&mut self,
                 resolver: &TextResolver,
                 current_locale: Locale,
                 content: impl Iterator<Item=TextContent>,
                 request: ScreenTextRequest) {
        let mut pen = self.screen_size * glm::vec2(request.screen_position_percentage.x, request.screen_position_percentage.y);

        for content in content {
            match content {
                TextContent::Number(number) => {
                    if let Some(character_iterator) = Base10CharIterator::new(number) {
                        self.queue_glyphs(resolver, &request, character_iterator, &mut pen);
                    }
                },
                TextContent::Text(text) => {
                    if let Some(text) = resolver.get_text(current_locale, text) {
                        self.queue_glyphs(resolver, &request, text.chars(), &mut pen);
                    }
                },
            }
        }
    }

    pub fn draw(&mut self, texture: &BitmapTexture) {
        self.shader_program.activate();
        self.attribute_program.activate();
        self.shader_program.set_vec2(UniformKey::ScreenWindowSize, self.screen_size);
        texture.activate();

        self.attr_pos.prepare_buffer();
        self.attr_glyph_size.prepare_buffer();
        self.attr_texel.prepare_buffer();
        self.attr_color.prepare_buffer();

        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.attr_pos.data.len() as GLsizei);
        }

        self.attr_pos.data.clear();
        self.attr_glyph_size.data.clear();
        self.attr_texel.data.clear();
        self.attr_color.data.clear();
    }

    fn queue_glyphs(&mut self,
                    resolver: &TextResolver,
                    request: &ScreenTextRequest,
                    chars: impl Iterator<Item=char>,
                    pen: &mut glm::Vec2) {
        for character in chars {
            if let Some(glyph_info) = resolver.get_glyph_info(GlyphId::new(character, request.raster_size)) {
                let raster_info = glyph_info.raster_info();
                let character_pen = *pen + glm::vec2(raster_info.left_side_bearing, raster_info.height_offset);

                if character != ' ' {
                    self.attr_pos.data.push(PositionAttr {
                        position: glm::vec3(character_pen.x, character_pen.y, request.screen_position_percentage.z),
                    });
                    self.attr_glyph_size.data.push(GlyphSizeAttr {
                        size: raster_info.raster_dimensions,
                    });
                    self.attr_texel.data.push(TexelAttr {
                        texel: glyph_info.texel(),
                    });
                    self.attr_color.data.push(ColorAttr {
                        color: glm::vec4(request.color.x, request.color.y, request.color.z, request.alpha)
                    });
                }

                pen.x += raster_info.advance_width;
            }
        }
    }
}

#[repr(C)]
struct GlyphSizeAttr {
    size: glm::Vec2,
}

impl attribute::KnownComponent for GlyphSizeAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct PositionAttr {
    position: glm::Vec3,
}

impl attribute::KnownComponent for PositionAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct ColorAttr {
    color: glm::Vec4,
}

impl attribute::KnownComponent for ColorAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct TexelAttr {
    texel: Texel,
}

impl attribute::KnownComponent for TexelAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::Float)
    }
}
