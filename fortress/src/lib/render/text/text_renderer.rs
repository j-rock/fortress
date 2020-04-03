use crate::{
    app::StatusOr,
    file::{
        self,
        ConfigWatcher,
        SimpleConfigManager,
    },
    math::Projections,
    render::{
        attribute,
        AttributeProgram,
        Attribute,
        GlyphTexture,
        ShaderProgram,
        ShaderUniformKey,
        TextureUnit,
        TextConfig,
        TextRenderOrder,
        TextWarehouse,
    },
};
use gl::{
    self,
    types::{
        GLint,
        GLsizei,
    },
};
use glm;
use glyph_brush::{
    self,
    BrushAction,
    BrushError,
    GlyphBrush,
    GlyphBrushBuilder,
};
use std::ffi::CString;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    RasterizedFontTexture,
    ProjectionMatrix,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let string = match self {
            UniformKey::RasterizedFontTexture => "font",
            UniformKey::ProjectionMatrix => "projection",
        };
        CString::new(string).expect("Bad cstring")
    }
}

pub struct TextRenderer {
    config: SimpleConfigManager<TextConfig>,
    text_warehouse: TextWarehouse,
    glyph_brush: GlyphBrush<'static, GlyphVertex>,
    glyph_texture: GlyphTexture,
    shader_program: ShaderProgram<UniformKey>,

    attribute_program: AttributeProgram,
    attr_screen_pos: Attribute<RectAttr>,
    attr_texel: Attribute<RectAttr>,
    attr_color: Attribute<ColorAttr>,
    attr_z_pos: Attribute<FloatAttr>,

    max_image_size: u32,
}

impl TextRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::<TextConfig>::from_config_resource(config_watcher, "text.conf")?;
        let text_warehouse = TextWarehouse::new(config.get());
        let glyph_brush = Self::make_glyph_brush(config.get());
        let glyph_texture = Self::make_glyph_texture(&glyph_brush);

        let vertex = file::util::resource_path("shaders", "text_vert.glsl");
        let geometry = file::util::resource_path("shaders", "text_geo.glsl");
        let fragment = file::util::resource_path("shaders", "text_frag.glsl");
        let mut shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        shader_program.activate();
        shader_program.set_texture(UniformKey::RasterizedFontTexture, TextureUnit::Texture0);

        let mut attribute_program_builder = AttributeProgram::builder();
        let attr_screen_pos = attribute_program_builder.add_attribute();
        let attr_texel = attribute_program_builder.add_attribute();
        let attr_color = attribute_program_builder.add_attribute();
        let attr_z_pos = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        let mut max_image_size: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_image_size)
        };

        Ok(TextRenderer {
            config,
            text_warehouse,
            glyph_brush,
            glyph_texture,
            shader_program,
            attribute_program,
            attr_screen_pos,
            attr_texel,
            attr_color,
            attr_z_pos,
            max_image_size: max_image_size as u32,
        })
    }

    pub fn pre_update(&mut self) {
        if self.config.update() {
            self.glyph_brush = Self::make_glyph_brush(self.config.get());
            self.glyph_texture = Self::make_glyph_texture(&self.glyph_brush);
            self.text_warehouse = TextWarehouse::new(self.config.get());
        }

        self.text_warehouse.clear_string_allocator();
    }

    pub fn queue(&mut self, text: TextRenderOrder) {
        let config = self.config.get();
        if let Some(section) = text.to_section(config.current_locale, &mut self.text_warehouse) {
            self.glyph_brush.queue(section);
        }
    }

    pub fn draw(&mut self, screen_size: glm::IVec2) {
        self.shader_program.activate();
        let transform = Projections::ortho(0.0, screen_size.x as f32, 0.0, screen_size.y as f32, -1.0, 1.0);
        self.shader_program.set_mat4(UniformKey::ProjectionMatrix, &transform);
        self.glyph_texture.activate();

        let update_texture = |rect: glyph_brush::rusttype::Rect<u32>, tex_data: &[u8]| {
            let (x, y) = (rect.min.x as i32, rect.min.y as i32);
            let (width, height) = (rect.width() as i32, rect.height() as i32);
            unsafe {
                gl::TexSubImage2D(gl::TEXTURE_2D, 0, x, y, width, height, gl::RED, gl::UNSIGNED_BYTE, tex_data.as_ptr() as _);
            }
        };
        let transform_vertex = |glyph| GlyphVertex::from(glyph);
        let mut resized_already = false;

        loop {
            match self.glyph_brush.process_queued(update_texture, transform_vertex) {
                Ok(brush_action) => {
                    self.finish_draw(brush_action);
                    return;
                },
                Err(BrushError::TextureTooSmall {
                    suggested, ..
                }) => {
                    if resized_already { return; }
                    self.resize_glyph_texture(suggested);
                    resized_already = true;
                }
            }
        }
    }

    fn finish_draw(&mut self, brush_action: BrushAction<GlyphVertex>) {
        match brush_action {
            BrushAction::Draw(vertices) => {
                self.attr_screen_pos.data.clear();
                self.attr_texel.data.clear();
                self.attr_color.data.clear();
                self.attr_z_pos.data.clear();

                for vertex in vertices.into_iter() {
                    self.attr_screen_pos.data.push(vertex.screen_rect);
                    self.attr_texel.data.push(vertex.texel_rect);
                    self.attr_color.data.push(vertex.color);
                    self.attr_z_pos.data.push(vertex.z_pos);
                }
            },
            BrushAction::ReDraw => {},
        }

        self.attribute_program.activate();
        self.glyph_texture.activate();
        self.attr_screen_pos.prepare_buffer();
        self.attr_texel.prepare_buffer();
        self.attr_color.prepare_buffer();
        self.attr_z_pos.prepare_buffer();
        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.attr_screen_pos.data.len() as GLsizei);
        }

        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }

    fn resize_glyph_texture(&mut self, suggested: (u32, u32)) {
        let curr_dim = self.glyph_brush.texture_dimensions();
        let curr_dim_small = curr_dim.0 < self.max_image_size || curr_dim.1 < self.max_image_size;
        let suggested_too_large = suggested.0 > self.max_image_size || suggested.1 > self.max_image_size;
        let new_dimensions = if curr_dim_small && suggested_too_large {
            (self.max_image_size, self.max_image_size)
        } else {
            suggested
        };
        println!("Had to resize glyph texture: {:?}", new_dimensions);
        self.glyph_brush.resize_texture(new_dimensions.0, new_dimensions.1);
        self.glyph_texture = Self::make_glyph_texture(&self.glyph_brush);
        self.glyph_texture.activate();
    }

    fn make_glyph_brush(config: &TextConfig) -> GlyphBrush<'static, GlyphVertex> {
        let font: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "\\fonts\\veger_regular.ttf"));
        GlyphBrushBuilder::using_font_bytes(font)
            .initial_cache_size(config.initial_glyph_cache_size)
            .build()
    }

    fn make_glyph_texture(glyph_brush: &GlyphBrush<'static, GlyphVertex>) -> GlyphTexture {
        let (width, height) = glyph_brush.texture_dimensions();
        GlyphTexture::new(glm::ivec2(width as i32, height as i32))
    }
}

#[derive(Clone, Debug)]
struct GlyphVertex {
    screen_rect: RectAttr,
    texel_rect: RectAttr,
    color: ColorAttr,
    z_pos: FloatAttr,
}

impl GlyphVertex {
    pub fn from(input: glyph_brush::GlyphVertex) -> Self {
        GlyphVertex {
            screen_rect: RectAttr {
                bottom_left: glm::vec2(input.pixel_coords.min.x as f32, input.pixel_coords.min.y as f32),
                top_right: glm::vec2(input.pixel_coords.max.x as f32, input.pixel_coords.max.y as f32),
            },
            texel_rect: RectAttr {
                bottom_left: glm::vec2(input.tex_coords.min.x, input.tex_coords.min.y),
                top_right: glm::vec2(input.tex_coords.max.x, input.tex_coords.max.y),
            },
            color: ColorAttr {
                color: glm::vec4(input.color[0], input.color[1], input.color[2], input.color[3]),
            },
            z_pos: FloatAttr {
                val: input.z,
            },
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
struct RectAttr {
    bottom_left: glm::Vec2,
    top_right: glm::Vec2,
}

impl attribute::KnownComponent for RectAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::Float)
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
struct ColorAttr {
    color: glm::Vec4,
}

impl attribute::KnownComponent for ColorAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::Float)
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
struct FloatAttr {
    val: f32,
}

impl attribute::KnownComponent for FloatAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::Float)
    }
}
