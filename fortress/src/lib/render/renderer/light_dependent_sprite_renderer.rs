use crate::{
    app::StatusOr,
    dimensions::Reverse,
    file,
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        CameraGeometry,
        CameraStreamInfo,
        NamedSpriteSheet,
        SpriteSheetFrameId,
        SpriteSheetTextureManager,
        PointLights,
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
use nalgebra::{
    self,
    Point2,
};
use std::{
    collections::HashMap,
    ffi::CString,
};

#[derive(Clone)]
pub struct LightDependentSpriteData {
    pub world_center_position: glm::Vec3,
    pub world_half_size: glm::Vec2,
    pub sprite_frame_id: SpriteSheetFrameId,
    pub frame: usize,
    pub unit_world_rotation: nalgebra::Vector2<f64>,
    pub reverse: Reverse,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    LightsPosition(usize),
    LightsColor(usize),
    LightsAttenuation(usize),
    NumLights,
    ProjectionView,
    PositionIndependentView,
    CameraRight,
    CameraUp,
    Texture(TextureUnit),
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        match self {
            UniformKey::NumLights => CString::new("num_lights").expect("Bad cstring"),
            UniformKey::LightsPosition(idx) => {
                let s = format!("lights[{}].position", idx);
                CString::new(s).expect("Bad cstring")
            },
            UniformKey::LightsColor(idx) => {
                let s = format!("lights[{}].color", idx);
                CString::new(s).expect("Bad cstring")
            },
            UniformKey::LightsAttenuation(idx) => {
                let s = format!("lights[{}].attenuation", idx);
                CString::new(s).expect("Bad cstring")
            },
            UniformKey::ProjectionView => CString::new("projection_view").expect("Bad cstring"),
            UniformKey::PositionIndependentView => CString::new("position_independent_view").expect("Bad cstring"),
            UniformKey::CameraRight => CString::new("camera_right").expect("Bad cstring"),
            UniformKey::CameraUp => CString::new("camera_up").expect("Bad cstring"),
            UniformKey::Texture(texture_unit) => CString::new(texture_unit.uniform_name()).expect("Bad cstring"),
        }
    }
}

pub struct LightDependentSpriteRenderer {
    shader_program: ShaderProgram<UniformKey>,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<SpritePositionAttr>,
    attr_size: Attribute<SpriteSizeAttr>,
    attr_texel: Attribute<Texel>,
    attr_rot: Attribute<RotationAttr>,
    per_pack_attrs: HashMap<NamedSpriteSheet, Vec<LightDependentSpriteData>>,
    camera_stream_info: Option<CameraStreamInfo>,
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
            camera_stream_info: None,
        })
    }

    pub fn set_camera_stream_info(&mut self, camera_stream_info: CameraStreamInfo) {
        self.camera_stream_info = Some(camera_stream_info);
    }

    pub fn queue(&mut self, datum: LightDependentSpriteData) {
        if let Some(ref camera_stream_info) = self.camera_stream_info {
            let world_position = Point2::new(datum.world_center_position.x as f64, -datum.world_center_position.z as f64);
            if !camera_stream_info.is_point_inside(world_position) {
                // Ignore points not inside camera stream inner bounds.
                return;
            }
        }

        self.per_pack_attrs
            .entry(datum.sprite_frame_id.sprite_sheet())
            .or_insert(Vec::new())
            .push(datum);
    }

    pub fn draw(&mut self, lights: &PointLights, textures: &SpriteSheetTextureManager, camera_geometry: &CameraGeometry) {
        self.shader_program.activate();
        self.attribute_program.activate();

        self.shader_program.set_mat4(UniformKey::ProjectionView, &camera_geometry.projection_view);
        self.shader_program.set_mat4(UniformKey::PositionIndependentView, &camera_geometry.isometric_view);
        self.shader_program.set_vec3(UniformKey::CameraRight, &camera_geometry.isometric_right);
        self.shader_program.set_vec3(UniformKey::CameraUp, &camera_geometry.isometric_up);
        self.shader_program.set_i32(UniformKey::NumLights, lights.len() as i32);

        for (idx, point_light) in lights.iter().enumerate() {
            self.shader_program.set_vec3(UniformKey::LightsPosition(idx), &point_light.shader_position());
            self.shader_program.set_vec3(UniformKey::LightsColor(idx), &point_light.shader_color());
            self.shader_program.set_vec3(UniformKey::LightsAttenuation(idx), &point_light.shader_attenuation());
        }

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
    unit_world_rotation_xz: glm::Vec2,
}

impl attribute::KnownComponent for RotationAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}
