use crate::{
    app::StatusOr,
    file,
    image::Png,
    render::{
        attribute,
        Attribute,
        AttributeAdvance,
        AttributeProgram,
        Texture,
        ShaderProgram,
    }
};

pub struct BackgroundRenderer {
    texture: Texture,
    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    attr_vertex: Attribute<VertexAttr>,
    attr_texel: Attribute<TexelAttr>,
}

impl BackgroundRenderer {
    pub fn new() -> StatusOr<BackgroundRenderer> {
        let vertex = file::util::resource_path("shaders", "background_vert.glsl");
        let fragment = file::util::resource_path("shaders", "background_frag.glsl");
        let shader_program = ShaderProgram::from_short_pipeline(&vertex, &fragment)?;

        let image_path = file::util::resource_path("images", "galaxy_ground.png");
        let image = Png::from_file(&image_path)?;
        let texture = Texture::new(image, 0);

        let mut attribute_program_builder = AttributeProgram::builder();
        let mut attr_vertex = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let mut attr_texel = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let attribute_program = attribute_program_builder.build();

        for vertex in [
            glm::vec2(-1.0,  1.0),
            glm::vec2(-1.0, -1.0),
            glm::vec2( 1.0,  1.0),
            glm::vec2( 1.0, -1.0)].into_iter() {
            attr_vertex.data.push( VertexAttr {
                vertex: *vertex
            });
        }

        for texel in [
            glm::vec2(0.0, 1.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(1.0, 0.0)].iter() {
            attr_texel.data.push( TexelAttr {
                texel: *texel,
            });
        }

        Ok(BackgroundRenderer {
            texture,
            shader_program,
            attribute_program,
            attr_vertex,
            attr_texel,
        })
    }

    pub fn draw(&mut self) {
        self.shader_program.activate();
        self.texture.activate(&mut self.shader_program);
        self.attribute_program.activate();
        self.attr_vertex.prepare_buffer();
        self.attr_texel.prepare_buffer();

        unsafe {
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}

#[repr(C)]
struct VertexAttr {
    vertex: glm::Vec2,
}

impl attribute::KnownComponent for VertexAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct TexelAttr {
    texel: glm::Vec2,
}

impl attribute::KnownComponent for TexelAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}

