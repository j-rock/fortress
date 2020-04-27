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

struct MeshInfo {
    vertices: Vec<glm::Vec3>,
    faces: Vec<GLuint>,
}

pub struct HexMesh {
    vbo: GLuint,
    ebo: GLuint,
    num_faces: usize,
}

impl HexMesh {
    pub fn new() -> Self {
        let mesh_info = Self::compute_mesh_info();

        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            let vertex_array_byte_size = (mesh_info.vertices.len() * std::mem::size_of::<glm::Vec3>()) as isize;
            gl::BufferData(gl::ARRAY_BUFFER, vertex_array_byte_size, mesh_info.vertices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            let faces_array_byte_size = (mesh_info.faces.len() * std::mem::size_of::<GLuint>()) as isize;
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, faces_array_byte_size, mesh_info.faces.as_ptr() as *const GLvoid, gl::STATIC_DRAW);

            let size_of_vec3_i32 = std::mem::size_of::<glm::Vec3>() as i32;
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of_vec3_i32, std::ptr::null());
        }

        HexMesh {
            vbo,
            ebo,
            num_faces: mesh_info.faces.len(),
        }
    }

    // Requires parent AttributeProgram to be active.
    pub fn draw(&self, num_instances: usize) {
        unsafe {
            gl::DrawElementsInstanced(gl::TRIANGLES, self.num_faces as GLsizei, gl::UNSIGNED_INT, std::ptr::null(), num_instances as GLsizei);
        }
    }

    fn compute_mesh_info() -> MeshInfo {
        let (back_left, back_right) = GridDirection::up().cartesian_offsets(1.0);
        let (right, front_right) = GridDirection::down_right().cartesian_offsets(1.0);
        let (front_left, left) = GridDirection::down_left().cartesian_offsets(1.0);

        let back_left = glm::vec3(back_left.x as f32, 0.0, -back_left.y as f32);
        let back_right = glm::vec3(back_right.x as f32, 0.0, -back_right.y as f32);
        let front_left = glm::vec3(front_left.x as f32, 0.0, -front_left.y as f32);
        let front_right = glm::vec3(front_right.x as f32, 0.0, -front_right.y as f32);
        let left = glm::vec3(left.x as f32, 0.0, -left.y as f32);
        let right = glm::vec3(right.x as f32, 0.0, -right.y as f32);

        let vertices = vec!(
            back_left,
            back_right,
            right,
            front_right,
            front_left,
            left,
            glm::vec3(right.x, -1.0, right.z),
            glm::vec3(front_right.x, -1.0, front_right.z),
            glm::vec3(front_left.x, -1.0, front_left.z),
            glm::vec3(left.x, -1.0, left.z));

        let faces = vec!(
            0, 3, 1,
            0, 4, 3,
            0, 5, 4,
            1, 3, 2,
            5, 9, 4,
            4, 9, 8,
            4, 8, 3,
            3, 8, 7,
            3, 7, 2,
            2, 7, 6);

        MeshInfo {
            vertices,
            faces,
        }
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
