use crate::{
    app::StatusOr,
    file,
    render::{
        attribute,
        Attribute,
        AttributeAdvance,
        AttributeProgram,
        BloomConfig,
        BloomPingPongBuffer,
        FrameBufferTexture,
        ShaderProgram,
        ShaderUniformKey,
        TextureUnit,
    },
};
use gl;
use std::ffi::CString;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    Bloom,
    BloomIntensityMultiplier,
    Scene,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let string = match self {
            UniformKey::Bloom => "bloom",
            UniformKey::BloomIntensityMultiplier => "bloom_intensity_multiplier",
            UniformKey::Scene => "scene",
        };
        CString::new(string).expect("Bad cstring")
    }
}

pub struct BloomCompositorShader {
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_position: Attribute<BloomPositionAttr>,
    attr_texel: Attribute<BloomTexelAttr>,
}

impl BloomCompositorShader {
    pub fn new() -> StatusOr<Self> {
        let vertex = file::util::resource_path("shaders", "bloom_compositor_vert.glsl");
        let fragment = file::util::resource_path("shaders", "bloom_compositor_frag.glsl");
        let mut shader_program = ShaderProgram::from_short_pipeline(&vertex, &fragment)?;
        shader_program.activate();
        shader_program.set_texture(UniformKey::Scene, TextureUnit::Texture0);
        shader_program.set_texture(UniformKey::Bloom, TextureUnit::Texture1);

        let mut attribute_program_builder = AttributeProgram::builder();
        let mut attr_position = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let mut attr_texel = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let attribute_program = attribute_program_builder.build();

        for (x, y) in [(-1.0,  1.0), (-1.0, -1.0), ( 1.0,  1.0), ( 1.0, -1.0)].iter() {
            attr_position.data.push( BloomPositionAttr {
                position: glm::vec3(*x, *y, 0.0)
            });
        }
        for (u, v) in [(0.0, 1.0), (0.0, 0.0), (1.0, 1.0), (1.0, 0.0)].iter() {
            attr_texel.data.push( BloomTexelAttr {
                texel: glm::vec2(*u, *v)
            });
        }

        shader_program.deactivate();

        Ok(BloomCompositorShader {
            shader_program,
            attribute_program,
            attr_position,
            attr_texel,
        })
    }

    pub fn draw(&mut self, config: &BloomConfig, scene: &FrameBufferTexture, bloom: &BloomPingPongBuffer) {
        self.shader_program.activate();
        self.shader_program.set_f32(UniformKey::BloomIntensityMultiplier, config.bloom_intensity_multiplier);

        TextureUnit::Texture0.activate();
        scene.bind();
        TextureUnit::Texture1.activate();
        bloom.bind_color_texture();

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
struct BloomPositionAttr {
    position: glm::Vec3,
}

impl attribute::KnownComponent for BloomPositionAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct BloomTexelAttr {
    texel: glm::Vec2,
}

impl attribute::KnownComponent for BloomTexelAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}
