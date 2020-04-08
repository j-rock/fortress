use crate::{
    app::StatusOr,
    file::{
        self,
        ConfigWatcher,
        SimpleConfigManager,
    },
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
        GlyphId,
        GlyphInfo,
        PackedGlyphSheet,
        TextConfig,
        TextContent,
        TextRenderRequest,
        TextSurface,
        TextWarehouse,
    },
};
use gl::{
    self,
    types::GLsizei,
};
use std::{
    collections::HashMap,
    ffi::CString,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum ScreenUniformKey {
    FontTexture,
    ScreenWindowSize,
}

impl ShaderUniformKey for ScreenUniformKey {
    fn to_cstring(self) -> CString {
        let string = match self {
            Self::FontTexture => "font",
            Self::ScreenWindowSize => "screen_window_size",
        };
        CString::new(string).expect("Bad cstring")
    }
}

pub struct TextRenderer {
    config: SimpleConfigManager<TextConfig>,
    text_warehouse: TextWarehouse,
    texture: BitmapTexture,
    mappings: HashMap<GlyphId, GlyphInfo>,

    screen_shader: ShaderProgram<ScreenUniformKey>,
    screen_attribute_program: AttributeProgram,
    screen_attr_pos: Attribute<PositionAttr>,
    screen_attr_glyph_size: Attribute<GlyphSizeAttr>,
    screen_attr_texel: Attribute<TexelAttr>,
    screen_attr_color: Attribute<ColorAttr>,
}

impl TextRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::from_config_resource(config_watcher, "text.conf")?;

        let (text_warehouse, texture, mappings) = {
            let config = config.get();
            let text_warehouse = TextWarehouse::new(config);
            let fonts = file::util::resource_base().join("fonts");
            let packed = PackedGlyphSheet::new(config, &fonts)?;
            let texture = BitmapTexture::new(packed.image, config.texture_atlas_style, TextureUnit::Texture0);
            (text_warehouse, texture, packed.mappings)
        };

        let mut screen_shader = {
            let vertex = file::util::resource_path("shaders", "screen_text_vert.glsl");
            let geometry = file::util::resource_path("shaders", "screen_text_geo.glsl");
            let fragment = file::util::resource_path("shaders", "screen_text_frag.glsl");
            ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?
        };
        screen_shader.activate();
        screen_shader.set_texture(ScreenUniformKey::FontTexture, TextureUnit::Texture0);

        let mut screen_attribute_program_builder = AttributeProgram::builder();
        let screen_attr_pos = screen_attribute_program_builder.add_attribute();
        let screen_attr_glyph_size = screen_attribute_program_builder.add_attribute();
        let screen_attr_texel = screen_attribute_program_builder.add_attribute();
        let screen_attr_color = screen_attribute_program_builder.add_attribute();
        let screen_attribute_program = screen_attribute_program_builder.build();

        Ok(TextRenderer {
            config,
            text_warehouse,
            texture,
            mappings,
            screen_shader,
            screen_attribute_program,
            screen_attr_pos,
            screen_attr_glyph_size,
            screen_attr_texel,
            screen_attr_color,
        })
    }

    pub fn pre_update(&mut self) {
        if self.config.update() {
            let config = self.config.get();
            let fonts = file::util::resource_base().join("fonts");
            match PackedGlyphSheet::new(config, &fonts) {
                Err(e) => println!("Couldn't reload text glyphs: {:?}", e),
                Ok(packed) => {
                    self.text_warehouse = TextWarehouse::new(config);
                    self.texture = BitmapTexture::new(packed.image, config.texture_atlas_style, TextureUnit::Texture0);
                    self.mappings = packed.mappings;
                },
            }
        }
    }

    pub fn draw(&mut self, screen_size: glm::IVec2) {
        self.screen_shader.activate();
        self.screen_attribute_program.activate();
        self.screen_shader.set_vec2(ScreenUniformKey::ScreenWindowSize, glm::vec2(screen_size.x as f32, screen_size.y as f32));
        self.texture.activate();

        self.screen_attr_pos.prepare_buffer();
        self.screen_attr_glyph_size.prepare_buffer();
        self.screen_attr_texel.prepare_buffer();
        self.screen_attr_color.prepare_buffer();

        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.screen_attr_pos.data.len() as GLsizei);
        }

        self.screen_attr_pos.data.clear();
        self.screen_attr_glyph_size.data.clear();
        self.screen_attr_texel.data.clear();
        self.screen_attr_color.data.clear();
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
    val: glm::Vec3,
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
