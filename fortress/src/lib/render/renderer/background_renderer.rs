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
        AttributeAdvance,
        AttributeProgram,
        NamedSpriteSheet,
        ShaderProgram,
        ShaderUniformKey,
        SpriteSheetTextureManager,
        Texture,
        TextureUnit,
    }
};
use std::ffi::CString;

#[derive(Deserialize)]
struct BackgroundRendererConfig {
    pub camera_speed: f32,
    // Screen pixels / pixels sampled.
    pub zoom: f32
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    Texture(TextureUnit)
}

impl ShaderUniformKey for UniformKey {
   fn to_cstring(self) -> CString {
       match self {
          UniformKey::Texture(texture_unit) => {
              CString::new(texture_unit.uniform_name()).expect("Bad texture")
          }
       }
   }
}

pub struct BackgroundRenderer {
    config_manager: SimpleConfigManager<BackgroundRendererConfig>,
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_vertex: Attribute<VertexAttr>,
    attr_texel: Attribute<TexelAttr>,
}

impl BackgroundRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<BackgroundRenderer> {
        let config_manager = SimpleConfigManager::from_config_resource(config_watcher, "background_renderer.conf")?;

        let vertex = file::util::resource_path("shaders", "background_vert.glsl");
        let fragment = file::util::resource_path("shaders", "background_frag.glsl");
        let shader_program = ShaderProgram::from_short_pipeline(&vertex, &fragment)?;

        let mut attribute_program_builder = AttributeProgram::builder();
        let mut attr_vertex = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let attr_texel = attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let attribute_program = attribute_program_builder.build();

        for vertex in [
            glm::vec2(-1.0,  1.0),
            glm::vec2(-1.0, -1.0),
            glm::vec2( 1.0,  1.0),
            glm::vec2( 1.0, -1.0)].iter() {
            attr_vertex.data.push( VertexAttr {
                vertex: *vertex
            });
        }

        Ok(BackgroundRenderer {
            config_manager,
            shader_program,
            attribute_program,
            attr_vertex,
            attr_texel,
        })
    }

    pub fn pre_update(&mut self) {
        self.config_manager.update();
    }

    pub fn draw(&mut self, textures: &SpriteSheetTextureManager, camera_pos: glm::Vec3) {
        let texture = textures.texture(NamedSpriteSheet::GalaxyGround);
        self.set_texels(texture, camera_pos);

        self.shader_program.activate();
        self.attribute_program.activate();
        self.attr_vertex.prepare_buffer();
        self.attr_texel.prepare_buffer();

        let texture_unit = texture.activate();
        self.shader_program.set_gluint(UniformKey::Texture(texture_unit), texture_unit.to_gluint());

        unsafe {
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }

     fn set_texels(&mut self, texture: &Texture, camera_pos: glm::Vec3) {
         self.attr_texel.data.clear();

         let config = self.config_manager.get();
         let (image_width, image_height) = texture.dimensions();
         let texel_width = config.zoom / image_width as f32;
         let texel_height = config.zoom / image_height as f32;

         let bottom_left = glm::vec2(camera_pos.x, -camera_pos.z) * config.camera_speed;
         let top_right = bottom_left + glm::vec2(texel_width, texel_height);

         for texel in [
             glm::vec2(bottom_left.x, top_right.y),
             bottom_left,
             top_right,
             glm::vec2(top_right.x, bottom_left.y)].iter() {
             self.attr_texel.data.push( TexelAttr {
                 texel: *texel,
             });
         }
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

