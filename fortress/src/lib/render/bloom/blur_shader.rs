use crate::{
    app::StatusOr,
    file,
    render::{
        attribute,
        Attribute,
        AttributeAdvance,
        AttributeProgram,
        ShaderProgram,
        ShaderUniformKey,
        TextureUnit,
    },
};
use gl;
use std::ffi::CString;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    HorizontalMode,
    ImageTexture,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let string = match self {
            UniformKey::HorizontalMode => "horizontal",
            UniformKey::ImageTexture => "image",
        };
        CString::new(string).expect("Bad cstring")
    }
}

pub struct BlurShader {
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_position: Attribute<BlurPositionAttr>,
    attr_texel: Attribute<BlurTexelAttr>,
}

impl BlurShader {
    pub fn new() -> StatusOr<Self> {
        let vertex = file::util::resource_path("shaders", "blur_vert.glsl");
        let fragment = file::util::resource_path("shaders", "blur_frag.glsl");
        let mut shader_program = ShaderProgram::from_short_pipeline(&vertex, &fragment)?;
        shader_program.activate();
        shader_program.set_texture(UniformKey::ImageTexture, TextureUnit::Texture0);

        let mut attribute_program_builder = AttributeProgram::builder();
        let mut attr_position = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let mut attr_texel = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let attribute_program = attribute_program_builder.build();

        for (x, y) in [(-1.0,  1.0), (-1.0, -1.0), ( 1.0,  1.0), ( 1.0, -1.0)].iter() {
            attr_position.data.push( BlurPositionAttr {
                position: glm::vec3(*x, *y, 0.0)
            });
        }
        for (u, v) in [(0.0, 1.0), (0.0, 0.0), (1.0, 1.0), (1.0, 0.0)].iter() {
            attr_texel.data.push( BlurTexelAttr {
                texel: glm::vec2(*u, *v)
            });
        }

        shader_program.deactivate();

        Ok(BlurShader {
            shader_program,
            attribute_program,
            attr_position,
            attr_texel,
        })
    }

    pub fn activate(&self) {
        self.shader_program.activate();
    }

    pub fn set_horizontal_mode(&mut self, horizontal: bool) {
        self.shader_program.set_bool(UniformKey::HorizontalMode, horizontal);
    }

    pub fn draw(&self) {
        self.attribute_program.activate();
        self.attr_position.prepare_buffer();
        self.attr_texel.prepare_buffer();
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
        self.attribute_program.deactivate();
    }
}

#[repr(C)]
struct BlurPositionAttr {
    position: glm::Vec3,
}

impl attribute::KnownComponent for BlurPositionAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct BlurTexelAttr {
    texel: glm::Vec2,
}

impl attribute::KnownComponent for BlurTexelAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}