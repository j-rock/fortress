use crate::{
    app::StatusOr,
    dimensions::{
        GridDirection,
        GridIndex,
        Reverse,
    },
    file,
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        InstancedMesh,
        NamedSpriteSheet,
        PointLight,
        ShaderProgram,
        ShaderUniformKey,
        SpriteSheetFrameId,
        SpriteSheetTextureManager,
        TextureUnit,
    }
};
use gl::types::GLuint;
use glm;
use std::ffi::CString;

pub struct HexData {
    pub position: GridIndex,
    pub height: f32,
    pub elevation: f32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UniformKey {
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
    mesh: InstancedMesh,
    attribute_program: AttributeProgram,
    attr_transform: Attribute<HexTransformAttr>,
    attr_scale: Attribute<HexScaleAttr>,

    tile_scale: glm::Vec2,
}

impl HexRenderer {
    pub fn new() -> StatusOr<HexRenderer> {
        let vertex = file::util::resource_path("shaders", "hex_vert.glsl");
        let geometry = file::util::resource_path("shaders", "hex_geo.glsl");
        let fragment = file::util::resource_path("shaders", "hex_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        // The InstancedMesh will take up the first vertex attrib slot.
        let mut attribute_program_builder = AttributeProgram::builder_with_offset(1);
        let attr_transform = attribute_program_builder.add_attribute();
        let attr_scale = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        let vertices = Self::compute_hexagon_vertices();
        let faces = Self::compute_hexagon_faces();
        let mesh = InstancedMesh::from_geometry(vertices, faces, &attribute_program);

        Ok(HexRenderer {
            shader_program,
            mesh,
            attribute_program,
            attr_transform,
            attr_scale,
            tile_scale: glm::vec2(1.0, 1.0),
        })
    }

    pub fn queue(&mut self, hex_cell_length: f64, data: &[HexData]) {
        let axial_to_cartesian = GridIndex::axial_to_cartesian(hex_cell_length);

        for datum in data.iter() {
            let hex_center = datum.position.index_center(&axial_to_cartesian);
            self.attr_transform.data.push(HexTransformAttr {
                position: glm::vec2(hex_center.x as f32, -hex_center.y as f32),
                height: datum.height,
                elevation: datum.elevation,
            });
            self.attr_scale.data.push(HexScaleAttr {
                scale: hex_cell_length as f32,
            });
        }
    }

    pub fn draw(&mut self, textures: &SpriteSheetTextureManager, lights: &Vec<PointLight>, projection_view: &glm::Mat4) {
        self.shader_program.activate();
        self.attribute_program.activate();
        self.attr_transform.prepare_buffer();
        self.attr_scale.prepare_buffer();

        let texture = textures.texture(NamedSpriteSheet::SpriteSheet1);
        let texture_unit = texture.activate();
        self.shader_program.set_gluint(UniformKey::Texture(texture_unit), texture_unit.to_gluint());

        let tile_frame_id = SpriteSheetFrameId {
            name: String::from("rock_texture.png"),
            sprite_sheet: NamedSpriteSheet::SpriteSheet1,
        };
        let texel = textures.frame(&tile_frame_id, 0, Reverse::none());

        self.shader_program.set_vec2(UniformKey::TileBottomLeft, texel.bottom_left);
        self.shader_program.set_vec2(UniformKey::TileTopRight, texel.top_right);
        self.shader_program.set_vec2(UniformKey::TileScale, self.tile_scale);
        self.shader_program.set_mat4(UniformKey::ProjectionView, projection_view);

        if lights.len() > 100 {
            panic!("Need to update shaders to support more than {} lights", lights.len());
        }
        self.shader_program.set_i32(UniformKey::NumLights, lights.len() as i32);
        for (idx, point_light) in lights.iter().enumerate() {
            self.shader_program.set_vec3(UniformKey::LightsPosition(idx), &point_light.position);
            self.shader_program.set_vec3(UniformKey::LightsColor(idx), &point_light.color);
            self.shader_program.set_vec3(UniformKey::LightsAttenuation(idx), &point_light.attenuation);
        }

        self.mesh.draw(self.attr_transform.data.len());

        self.attribute_program.deactivate();
        self.shader_program.deactivate();
        self.attr_transform.data.clear();
        self.attr_scale.data.clear();
    }

    pub fn set_tile_scale(&mut self, tile_scale: glm::Vec2) {
        self.tile_scale = tile_scale;
    }

    fn compute_hexagon_vertices() -> Vec<glm::Vec3> {
        let (vec2_0, vec2_1) = GridDirection::up().cartesian_offsets(1.0);
        let (vec2_2, vec2_3) = GridDirection::down_right().cartesian_offsets(1.0);
        let (vec2_4, vec2_5) = GridDirection::down_left().cartesian_offsets(1.0);

        vec!(
            glm::vec3(vec2_0.x as f32, 0.0,  -vec2_0.y as f32),
            glm::vec3(vec2_1.x as f32, 0.0 , -vec2_1.y as f32),
            glm::vec3(vec2_2.x as f32, 0.0 , -vec2_2.y as f32),
            glm::vec3(vec2_3.x as f32, 0.0 , -vec2_3.y as f32),
            glm::vec3(vec2_4.x as f32, 0.0 , -vec2_4.y as f32),
            glm::vec3(vec2_5.x as f32, 0.0 , -vec2_5.y as f32),
            glm::vec3(vec2_2.x as f32, -1.0, -vec2_2.y as f32),
            glm::vec3(vec2_3.x as f32, -1.0, -vec2_3.y as f32),
            glm::vec3(vec2_4.x as f32, -1.0, -vec2_4.y as f32),
            glm::vec3(vec2_5.x as f32, -1.0, -vec2_5.y as f32),
        )
    }

    fn compute_hexagon_faces() -> Vec<GLuint> {
        vec!(
            0, 3, 1,
            0, 4, 3,
            0, 5, 4,
            1, 3, 2,
            5, 9, 4,
            4, 9, 8,
            4, 8, 3,
            3, 8, 7,
            3, 7, 2,
            2, 7, 6
        )
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
