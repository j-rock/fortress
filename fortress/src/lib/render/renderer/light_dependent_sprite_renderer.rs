use crate::{
    app::StatusOr,
    dimensions::Reverse,
    file,
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        NamedSpriteSheet,
        SpriteSheetFrameId,
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
pub struct LightDependentSpriteData {
    pub world_center_position: glm::Vec3,
    pub world_half_size: glm::Vec2,
    pub sprite_frame_id: SpriteSheetFrameId,
    pub frame: usize,
    pub rotation: f32,
    pub reverse: Reverse,
}

pub struct LightDependentSpriteRenderer {
    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<SpritePositionAttr>,
    attr_size: Attribute<SpriteSizeAttr>,
    attr_texel: Attribute<Texel>,
    attr_rot: Attribute<RotationAttr>,
    per_pack_attrs: HashMap<NamedSpriteSheet, Vec<LightDependentSpriteData>>,
}

impl LightDependentSpriteRenderer {
    pub fn new() -> StatusOr<LightDependentSpriteRenderer> {
        let vertex = file::util::resource_path("shaders", "light_dependent_sprite_vert.glsl");
        let geometry = file::util::resource_path("shaders", "light_dependent_sprite_geo.glsl");
        let fragment = file::util::resource_path("shaders", "light_dependent_sprite_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        let mut attribute_program_builder = AttributeProgram::builder();
        let attr_pos = attribute_program_builder.add_attribute();
        let attr_size = attribute_program_builder.add_attribute();
        let attr_texel = attribute_program_builder.add_attribute();
        let attr_rot = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(LightDependentSpriteRenderer {
            shader_program,
            attribute_program,
            attr_pos,
            attr_size,
            attr_texel,
            attr_rot,
            per_pack_attrs: HashMap::new(),
        })
    }

    pub fn queue(&mut self, data: Vec<LightDependentSpriteData>) {
        for datum in data.into_iter() {
            let pack_attrs = self.per_pack_attrs
                .entry(datum.sprite_frame_id.sprite_sheet)
                .or_insert(Vec::new());

            pack_attrs.push(datum);
        }
    }

    pub fn draw(&mut self, lights: &Vec<PointLight>, textures: &SpriteSheetTextureManager, projection_view: &glm::Mat4, camera_right: glm::Vec3, camera_up: glm::Vec3) {
        self.shader_program.activate();
        self.attribute_program.activate();

        self.shader_program.set_mat4("projection_view", projection_view);
        self.shader_program.set_vec3("camera_right", &camera_right);
        self.shader_program.set_vec3("camera_up", &camera_up);
        PointLight::set_lights(lights, &mut self.shader_program);

        for (named_texture, queued_draw) in self.per_pack_attrs.iter() {
            let texture = textures.texture(*named_texture);
            texture.activate(&mut self.shader_program);

            for datum in queued_draw.iter() {
                let texel = textures.frame(&datum.sprite_frame_id, datum.frame, datum.reverse);

                self.attr_pos.data.push(SpritePositionAttr {
                    world_center_position: datum.world_center_position,
                });
                self.attr_size.data.push(SpriteSizeAttr {
                    world_half_size: datum.world_half_size,
                });
                self.attr_texel.data.push(texel);
                self.attr_rot.data.push(RotationAttr {
                    rotation: datum.rotation
                });
            }

            self.attr_pos.prepare_buffer();
            self.attr_size.prepare_buffer();
            self.attr_texel.prepare_buffer();
            self.attr_rot.prepare_buffer();

            unsafe {
                gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.attr_pos.data.len() as GLsizei);
            }

            self.attr_pos.data.clear();
            self.attr_size.data.clear();
            self.attr_texel.data.clear();
            self.attr_rot.data.clear();
        }

        self.per_pack_attrs.clear();
        self.attribute_program.deactivate();
        self.shader_program.deactivate();
    }
}

#[repr(C)]
struct SpritePositionAttr {
    world_center_position: glm::Vec3,
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

#[repr(C)]
struct RotationAttr {
    rotation: f32,
}

impl attribute::KnownComponent for RotationAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::Float)
    }
}
