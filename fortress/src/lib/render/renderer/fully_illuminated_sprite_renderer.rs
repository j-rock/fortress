use crate::{
    app::StatusOr,
    dimensions::Reverse,
    file,
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        CameraGeometry,
        NamedSpriteSheet,
        SpriteSheetFrameId,
        SpriteSheetTextureManager,
        ShaderProgram,
        ShaderUniformKey,
        Texel,
        TextureUnit,
    }
};
use gl::{
    self,
    types::*
};
use glm;
use nalgebra;
use std::{
    collections::HashMap,
    ffi::CString
};

#[derive(Clone)]
pub struct FullyIlluminatedSpriteData {
    pub world_center_position: glm::Vec3,
    pub world_half_size: glm::Vec2,
    pub sprite_frame_id: SpriteSheetFrameId,
    pub frame: usize,
    pub unit_world_rotation: nalgebra::Vector2<f64>,
    pub reverse: Reverse,
    pub bloom_intensity: f32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    ProjectionView,
    PositionIndependentView,
    CameraRight,
    CameraUp,
    Texture(TextureUnit)
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        let string = match self {
            UniformKey::ProjectionView => "projection_view",
            UniformKey::PositionIndependentView => "position_independent_view",
            UniformKey::CameraRight => "camera_right",
            UniformKey::CameraUp => "camera_up",
            UniformKey::Texture(texture_unit) => texture_unit.uniform_name(),
        };
        CString::new(string).expect("Bad cstring")
    }
}

pub struct FullyIlluminatedSpriteRenderer {
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<SpritePositionAttr>,
    attr_size: Attribute<SpriteSizeAttr>,
    attr_texel: Attribute<Texel>,
    attr_rot: Attribute<RotationAttr>,
    attr_bloom_intensity: Attribute<BloomIntensityAttr>,
    per_pack_attrs: HashMap<NamedSpriteSheet, Vec<FullyIlluminatedSpriteData>>,
}

impl FullyIlluminatedSpriteRenderer {
    pub fn new() -> StatusOr<FullyIlluminatedSpriteRenderer> {
        let vertex = file::util::resource_path("shaders", "full_light_sprite_vert.glsl");
        let geometry = file::util::resource_path("shaders", "full_light_sprite_geo.glsl");
        let fragment = file::util::resource_path("shaders", "full_light_sprite_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        let mut attribute_program_builder = AttributeProgram::builder();
        let attr_pos = attribute_program_builder.add_attribute();
        let attr_size = attribute_program_builder.add_attribute();
        let attr_texel = attribute_program_builder.add_attribute();
        let attr_rot = attribute_program_builder.add_attribute();
        let attr_bloom_intensity = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(FullyIlluminatedSpriteRenderer {
            shader_program,
            attribute_program,
            attr_pos,
            attr_size,
            attr_texel,
            attr_rot,
            attr_bloom_intensity,
            per_pack_attrs: HashMap::new(),
        })
    }

    pub fn queue(&mut self, data: impl IntoIterator<Item = FullyIlluminatedSpriteData>) {
        for datum in data {
            let pack_attrs = self.per_pack_attrs
                .entry(datum.sprite_frame_id.sprite_sheet())
                .or_insert(Vec::new());

            pack_attrs.push(datum);
        }
    }

    pub fn draw(&mut self, textures: &SpriteSheetTextureManager, camera_geometry: &CameraGeometry) {
        self.shader_program.activate();
        self.attribute_program.activate();

        self.shader_program.set_mat4(UniformKey::ProjectionView, &camera_geometry.projection_view);
        self.shader_program.set_mat4(UniformKey::PositionIndependentView, &camera_geometry.isometric_view);
        self.shader_program.set_vec3(UniformKey::CameraRight, &camera_geometry.isometric_right);
        self.shader_program.set_vec3(UniformKey::CameraUp, &camera_geometry.isometric_up);

        for (named_texture, queued_draw) in self.per_pack_attrs.iter() {
            let texture = textures.texture(*named_texture);
            let texture_unit = texture.activate();
            self.shader_program.set_texture(UniformKey::Texture(texture_unit), texture_unit);

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
                    unit_world_rotation_xz: glm::vec2(datum.unit_world_rotation.x as f32, -datum.unit_world_rotation.y as f32)
                });
                self.attr_bloom_intensity.data.push(BloomIntensityAttr {
                    intensity: datum.bloom_intensity,
                })
            }

            self.attr_pos.prepare_buffer();
            self.attr_size.prepare_buffer();
            self.attr_texel.prepare_buffer();
            self.attr_rot.prepare_buffer();
            self.attr_bloom_intensity.prepare_buffer();

            unsafe {
                gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.attr_pos.data.len() as GLsizei);
            }

            self.attr_pos.data.clear();
            self.attr_size.data.clear();
            self.attr_texel.data.clear();
            self.attr_rot.data.clear();
            self.attr_bloom_intensity.data.clear();
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
    unit_world_rotation_xz: glm::Vec2,
}

impl attribute::KnownComponent for RotationAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct BloomIntensityAttr {
    intensity: f32,
}

impl attribute::KnownComponent for BloomIntensityAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::Float)
    }
}
