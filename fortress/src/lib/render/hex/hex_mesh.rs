use crate::dimensions::GridDirection;
use gl::{
    self,
    types::{
        GLsizei,
        GLuint,
        GLvoid,
    }
};
use glm;

pub struct HexMesh {
    vbo: GLuint,
    ebo: GLuint,
    num_faces: usize,
}

impl HexMesh {
    pub fn new() -> Self {
        let vertices = Self::compute_hexagon_vertices();
        let faces = Self::compute_hexagon_faces();

        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            let vertex_array_byte_size = (vertices.len() * std::mem::size_of::<glm::Vec3>()) as isize;
            gl::BufferData(gl::ARRAY_BUFFER, vertex_array_byte_size, vertices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            let faces_array_byte_size = (faces.len() * std::mem::size_of::<GLuint>()) as isize;
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, faces_array_byte_size, faces.as_ptr() as *const GLvoid, gl::STATIC_DRAW);

            let size_of_vec3_i32 = std::mem::size_of::<glm::Vec3>() as i32;
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of_vec3_i32, std::ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        HexMesh {
            vbo,
            ebo,
            num_faces: faces.len(),
        }
    }

    // Requires parent AttributeProgram to be active.
    pub fn draw(&self, num_instances: usize) {
        unsafe {
            gl::DrawElementsInstanced(gl::TRIANGLES, self.num_faces as GLsizei, gl::UNSIGNED_INT, std::ptr::null(), num_instances as GLsizei);
        }
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

impl Drop for HexMesh {
    fn drop(&mut self) {
        unsafe {
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
            if self.ebo != 0 {
                gl::DeleteBuffers(1, &self.ebo);
            }
        }
    }
}
