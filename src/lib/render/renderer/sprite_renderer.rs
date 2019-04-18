use crate::{
    app::StatusOr,
    file::{
        self,
        ConfigWatcher,
    },
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        NamedSpriteSheet,
        SpriteSheetTexelId,
        SpriteSheetTextureManager,
        PointLight,
        ShaderProgram,
        Texel,
    }
};
use gl::{
    self,
    types::*
};
use glm;
use hashbrown::HashMap;

#[derive(Clone)]
pub struct SpriteData {
    pub world_bottom_center_position: glm::Vec3,
    pub world_half_size: glm::Vec2,
    pub sprite_texel_id: SpriteSheetTexelId,
}

pub struct SpriteRenderer {
    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<SpritePositionAttr>,
    attr_size: Attribute<SpriteSizeAttr>,
    attr_texel: Attribute<Texel>,
    textures: SpriteSheetTextureManager,
    per_pack_attrs: HashMap<NamedSpriteSheet, Vec<SpriteData>>,
}

impl SpriteRenderer {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<SpriteRenderer> {
        let vertex = file::util::resource_path("shaders", "sprite_vert.glsl");
        let geometry = file::util::resource_path("shaders", "sprite_geo.glsl");
        let fragment = file::util::resource_path("shaders", "sprite_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        let mut attribute_program_builder = AttributeProgram::builder();
        let attr_pos = attribute_program_builder.add_attribute();
        let attr_size = attribute_program_builder.add_attribute();
        let attr_texel = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        let textures = SpriteSheetTextureManager::new(config_watcher)?;

        Ok(SpriteRenderer {
            shader_program,
            attribute_program,
            attr_pos,
            attr_size,
            attr_texel,
            textures,
            per_pack_attrs: HashMap::new(),
        })
    }

    pub fn update(&mut self) {
        self.textures.update();
    }

    pub fn queue(&mut self, data: Vec<SpriteData>) {
        for datum in data.into_iter() {
            let pack_attrs = self.per_pack_attrs
                .entry(datum.sprite_texel_id.sprite_sheet)
                .or_insert(Vec::new());

            pack_attrs.push(datum);
        }
    }

    pub fn draw(&mut self, lights: &Vec<PointLight>, projection_view: &glm::Mat4, camera_right: glm::Vec3, camera_up: glm::Vec3) {
        self.shader_program.activate();
        self.attribute_program.activate();

        self.shader_program.set_mat4("projection_view", projection_view);
        self.shader_program.set_vec3("camera_right", &camera_right);
        self.shader_program.set_vec3("camera_up", &camera_up);
        PointLight::set_lights(lights, &mut self.shader_program);

        for (named_texture, queued_draw) in self.per_pack_attrs.iter() {
            let texture = self.textures.texture(*named_texture);
            texture.activate(&mut self.shader_program);

            for datum in queued_draw.iter() {
                let texel = self.textures.texel(&datum.sprite_texel_id);

                self.attr_pos.data.push(SpritePositionAttr {
                    world_bottom_center_position: datum.world_bottom_center_position,
                });
                self.attr_size.data.push(SpriteSizeAttr {
                    world_half_size: datum.world_half_size,
                });
                self.attr_texel.data.push(*texel);
            }

            self.attr_pos.prepare_buffer();
            self.attr_size.prepare_buffer();
            self.attr_texel.prepare_buffer();

            unsafe {
                gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.attr_pos.data.len() as GLsizei);
            }

            self.attr_pos.data.clear();
            self.attr_size.data.clear();
            self.attr_texel.data.clear();
        }

        self.per_pack_attrs.clear();
        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}

#[repr(C)]
struct SpritePositionAttr {
    world_bottom_center_position: glm::Vec3,
}

impl attribute::KnownComponent for SpritePositionAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct SpriteSizeAttr {
    world_half_size: glm::Vec2,
}

impl attribute::KnownComponent for SpriteSizeAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}
