use crate::{
    app::StatusOr,
    dimensions::{
        GridDirection,
        GridIndex,
    },
    file,
    render::{
        attribute,
        Attribute,
        AttributeProgram,
        InstancedMesh,
        ShaderProgram,
    }
};
use gl::types::GLuint;
use glm;

pub struct HexData {
    pub position: GridIndex,
    pub height: f32,
    pub top_y_coord: f32,
    pub rgba_color: glm::Vec4,
}

pub struct HexRenderer {
    shader_program: ShaderProgram,
    // InstancedMesh should be destructed before AttributeProgram.
    mesh: InstancedMesh,
    attribute_program: AttributeProgram,
    attr_position: Attribute<HexPositionAttr>,
    attr_height: Attribute<HexHeightAttr>,
    attr_color: Attribute<HexColorAttr>,
}

impl HexRenderer {
    pub fn new() -> StatusOr<HexRenderer> {
        let vertex = file::util::resource_path("shaders", "hex_vert.glsl");
        let geometry = file::util::resource_path("shaders", "hex_geo.glsl");
        let fragment = file::util::resource_path("shaders", "hex_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        // The InstancedMesh will take up the first vertex attrib slot.
        let mut attribute_program_builder = AttributeProgram::builder_with_offset(1);
        let attr_position = attribute_program_builder.add_attribute();
        let attr_height = attribute_program_builder.add_attribute();
        let attr_color = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        let vertices = Self::compute_hexagon_vertices();
        let faces = Self::compute_hexagon_faces();
        let mesh = InstancedMesh::from_geometry(vertices, faces, &attribute_program);

        Ok(HexRenderer {
            shader_program,
            mesh,
            attribute_program,
            attr_position,
            attr_height,
            attr_color
        })
    }

    pub fn queue(&mut self, hex_cell_length: f64, data: &[HexData]) {
        let axial_to_cartesian = GridIndex::axial_to_cartesian(hex_cell_length);

        for datum in data.iter() {
            let hex_center = datum.position.index_center(&axial_to_cartesian);

            self.attr_position.data.push(HexPositionAttr {
                position: glm::vec3(hex_center.x as f32, datum.top_y_coord, -hex_center.y as f32)
            });
            self.attr_height.data.push(HexHeightAttr {
                height: datum.height,
            });
            self.attr_color.data.push(HexColorAttr {
                rgba_color: datum.rgba_color
            });
        }
    }

    pub fn draw_begin(&mut self) {
        self.shader_program.activate();
        self.attribute_program.activate();
        self.attr_position.prepare_buffer();
        self.attr_height.prepare_buffer();
        self.attr_color.prepare_buffer();
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        self.shader_program.set_mat4("projection_view", projection_view);
        self.mesh.draw(self.attr_position.data.len());
    }

    pub fn draw_end(&mut self) {
        self.attribute_program.deactivate();
        self.shader_program.deactivate();

        self.attr_position.data.clear();
        self.attr_height.data.clear();
        self.attr_color.data.clear();
    }

    fn compute_hexagon_vertices() -> Vec<glm::Vec3> {
        let (vec2_0, vec2_1) = GridDirection::up().cartesian_offsets(1.0);
        let (vec2_2, vec2_3) = GridDirection::down_right().cartesian_offsets(1.0);
        let (vec2_5, vec2_4) = GridDirection::down_left().cartesian_offsets(1.0);

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
struct HexPositionAttr {
    position: glm::Vec3,
}

impl attribute::KnownComponent for HexPositionAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::FLOAT)
    }
}

#[repr(C)]
struct HexHeightAttr {
    height: f32,
}

impl attribute::KnownComponent for HexHeightAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S1, attribute::ComponentType::FLOAT)
    }
}

#[repr(C)]
struct HexColorAttr {
    rgba_color: glm::Vec4,
}

impl attribute::KnownComponent for HexColorAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

