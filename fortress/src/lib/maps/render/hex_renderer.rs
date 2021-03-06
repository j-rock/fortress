use crate::{
    app::StatusOr,
    dimensions::{
        GridIndex,
        Reverse,
    },
    file,
    maps::{
        MapConfig,
        render::HexMesh
    },
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        CameraGeometry,
        NamedSpriteSheet,
        PointLights,
        ShaderProgram,
        ShaderUniformKey,
        SpriteSheetFrameId,
        SpriteSheetTextureManager,
        TextureUnit,
    }
};
use glm;
use std::ffi::CString;

pub struct HexData {
    pub position: GridIndex,
    pub height: f32,
    pub elevation: f32,
    pub alpha: f32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
    BevelRaise,
    LightsPosition(usize),
    LightsColor(usize),
    LightsAttenuation(usize),
    NumLights,
    Texture(TextureUnit),
    TileBottomLeft,
    TileTopRight,
    TileScale,
    ProjectionView,
}

impl ShaderUniformKey for UniformKey {
    fn to_cstring(self) -> CString {
        match self {
            UniformKey::BevelRaise => CString::new("bevel_raise").expect("Bad cstring"),
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
            UniformKey::NumLights => CString::new("num_lights").expect("Bad cstring"),
            UniformKey::Texture(texture_unit) => CString::new(texture_unit.uniform_name()).expect("Bad cstring"),
            UniformKey::TileBottomLeft => CString::new("tile_bottom_left").expect("Bad cstring"),
            UniformKey::TileTopRight => CString::new("tile_top_right").expect("Bad cstring"),
            UniformKey::TileScale => CString::new("tile_scale").expect("Bad cstring"),
            UniformKey::ProjectionView => CString::new("projection_view").expect("Bad cstring"),
        }
    }
}

pub struct HexRenderer {
    shader_program: ShaderProgram<UniformKey>,
    // InstancedMesh should be destructed before AttributeProgram.
    mesh: HexMesh,
    attribute_program: AttributeProgram,
    attr_transform: Attribute<HexTransformAttr>,
    attr_scale: Attribute<HexScaleAttr>,
    attr_alpha: Attribute<HexAlphaAttr>,
}

impl HexRenderer {
    pub fn new(config: &MapConfig) -> StatusOr<HexRenderer> {
        let vertex = file::util::resource_path("shaders", "hex_vert.glsl");
        let geometry = file::util::resource_path("shaders", "hex_geo.glsl");
        let fragment = file::util::resource_path("shaders", "hex_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        // The HexMesh will take up the first vertex attrib slot.
        let mut attribute_program_builder = AttributeProgram::builder_with_offset(1);
        // Important to create HexMesh while attribute program is active.
        let mesh = HexMesh::new(config);
        let attr_transform = attribute_program_builder.add_attribute();
        let attr_scale = attribute_program_builder.add_attribute();
        let attr_alpha = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(HexRenderer {
            shader_program,
            mesh,
            attribute_program,
            attr_transform,
            attr_scale,
            attr_alpha,
        })
    }

    pub fn queue(&mut self, hex_cell_length: f64, data: impl Iterator<Item = HexData>) {
        let axial_to_cartesian = GridIndex::axial_to_cartesian(hex_cell_length);

        for datum in data {
            let hex_center = datum.position.index_center(&axial_to_cartesian);
            self.attr_transform.data.push(HexTransformAttr {
                position: glm::vec2(hex_center.x as f32, -hex_center.y as f32),
                height: datum.height,
                elevation: datum.elevation,
            });
            self.attr_scale.data.push(HexScaleAttr {
                scale: hex_cell_length as f32,
            });
            self.attr_alpha.data.push(HexAlphaAttr {
                alpha: datum.alpha,
            });
        }
    }

    pub fn draw(&mut self, config: &MapConfig, textures: &SpriteSheetTextureManager, lights: &PointLights, camera_geometry: &CameraGeometry) {
        self.shader_program.activate();
        self.attribute_program.activate();
        self.attr_transform.prepare_buffer();
        self.attr_scale.prepare_buffer();
        self.attr_alpha.prepare_buffer();

        let texture = textures.texture(NamedSpriteSheet::SpriteSheet1);
        let texture_unit = texture.activate();
        self.shader_program.set_texture(UniformKey::Texture(texture_unit), texture_unit);

        let tile_frame_id = SpriteSheetFrameId::new(String::from("rock_texture.png"), NamedSpriteSheet::SpriteSheet1);
        let texel = textures.frame(&tile_frame_id, 0, Reverse::none());

        self.shader_program.set_f32(UniformKey::BevelRaise, config.bevel_height);
        self.shader_program.set_vec2(UniformKey::TileBottomLeft, texel.bottom_left);
        self.shader_program.set_vec2(UniformKey::TileTopRight, texel.top_right);
        self.shader_program.set_vec2(UniformKey::TileScale, glm::vec2(config.tile_scale.0, config.tile_scale.1));
        self.shader_program.set_mat4(UniformKey::ProjectionView, &camera_geometry.projection_view);
        self.shader_program.set_i32(UniformKey::NumLights, lights.len() as i32);
        for (idx, point_light) in lights.iter().enumerate() {
            self.shader_program.set_vec3(UniformKey::LightsPosition(idx), &point_light.shader_position());
            self.shader_program.set_vec3(UniformKey::LightsColor(idx), &point_light.shader_color());
            self.shader_program.set_vec3(UniformKey::LightsAttenuation(idx), &point_light.shader_attenuation());
        }

        self.mesh.draw(self.attr_transform.data.len());

        self.attribute_program.deactivate();
        self.shader_program.deactivate();
        self.attr_transform.data.clear();
        self.attr_scale.data.clear();
        self.attr_alpha.data.clear();
    }
}

#[repr(C)]
#[derive(Debug)]
struct HexTransformAttr {
    position: glm::Vec2,
    height: f32,
    elevation: f32,
}

impl attribute::KnownComponent for HexTransformAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct HexScaleAttr {
    scale: f32,
}

impl attribute::KnownComponent for HexScaleAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct HexAlphaAttr {
    alpha: f32,
}

impl attribute::KnownComponent for HexAlphaAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::Float)
    }
}
