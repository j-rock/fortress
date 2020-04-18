use crate::{
    app::StatusOr,
    file,
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        BitmapTexture,
        CameraGeometry,
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
        TextResolver,
        WorldTextRequest,
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
    CameraRight,
    CameraUp,
    FontTexture,
    ProjectionView,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let string = match self {
            Self::CameraRight => "camera_right",
            Self::CameraUp => "camera_up",
            Self::FontTexture => "font",
            Self::ProjectionView => "projection_view",
        };
        CString::new(string).expect("Bad cstring")
    }
}

pub struct WorldTextRenderer {
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<PositionAttr>,
    attr_glyph_size: Attribute<GlyphSizeAttr>,
    attr_texel: Attribute<TexelAttr>,
    attr_color: Attribute<ColorAttr>,

    camera_right: glm::Vec3,
    camera_up: glm::Vec3,
}

impl WorldTextRenderer {
    pub fn new() -> StatusOr<Self> {
        let vertex = file::util::resource_path("shaders", "world_text_vert.glsl");
        let geometry = file::util::resource_path("shaders", "world_text_geo.glsl");
        let fragment = file::util::resource_path("shaders", "world_text_frag.glsl");
        let mut shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;
        shader_program.activate();
        shader_program.set_texture(UniformKey::FontTexture, TextureUnit::Texture0);

        let mut attribute_program_builder = AttributeProgram::builder();
        let attr_pos = attribute_program_builder.add_attribute();
        let attr_glyph_size = attribute_program_builder.add_attribute();
        let attr_texel = attribute_program_builder.add_attribute();
        let attr_color = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(WorldTextRenderer {
            shader_program,
            attribute_program,
            attr_pos,
            attr_glyph_size,
            attr_texel,
            attr_color,
            camera_right: glm::vec3(1.0, 0.0, 0.0),
            camera_up: glm::vec3(0.0, 1.0, 0.0),
        })
    }

    pub fn set_camera_geometry(&mut self, camera_geometry: &CameraGeometry) {
        self.camera_right = camera_geometry.isometric_right;
        self.camera_up = camera_geometry.isometric_up;
    }

    pub fn queue(&mut self,
                 resolver: &TextResolver,
                 current_locale: Locale,
                 content: impl Iterator<Item=TextContent>,
                 request: WorldTextRequest) {
        let mut pen = request.world_position;

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

    pub fn draw(&mut self, texture: &BitmapTexture, camera_geometry: &CameraGeometry) {
        self.shader_program.activate();
        self.attribute_program.activate();
        self.shader_program.set_vec3(UniformKey::CameraRight, &self.camera_right);
        self.shader_program.set_vec3(UniformKey::CameraUp, &self.camera_up);
        self.shader_program.set_mat4(UniformKey::ProjectionView, &camera_geometry.projection_view);
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
                    request: &WorldTextRequest,
                    chars: impl Iterator<Item=char>,
                    pen: &mut glm::Vec3) {
        for character in chars {
            if let Some(glyph_info) = resolver.get_glyph_info(GlyphId::new(character, request.raster_size)) {
                let raster_info = glyph_info.raster_info();

                if character != ' ' {
                    self.attr_pos.data.push(PositionAttr {
                        position: {
                            let right = self.camera_right * raster_info.left_side_bearing;
                            let up = self.camera_up * raster_info.height_offset;
                            *pen + right + up
                        }
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

                *pen = *pen + self.camera_right * raster_info.advance_width;
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
