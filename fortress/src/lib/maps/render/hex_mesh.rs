use crate::{
    dimensions::GridDirection,
    maps::MapConfig,
};
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
    pub fn new(config: &MapConfig) -> Self {
        let mesh_info = Self::compute_mesh_info(config);

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

    fn compute_mesh_info(config: &MapConfig) -> MeshInfo {
        let vertices = {
            let (back_left, back_right) = GridDirection::up().cartesian_offsets(1.0);
            let (right, front_right) = GridDirection::down_right().cartesian_offsets(1.0);
            let (front_left, left) = GridDirection::down_left().cartesian_offsets(1.0);

            let back_left = glm::vec3(back_left.x as f32, 0.0, -back_left.y as f32);
            let back_right = glm::vec3(back_right.x as f32, 0.0, -back_right.y as f32);
            let front_left = glm::vec3(front_left.x as f32, 0.0, -front_left.y as f32);
            let front_right = glm::vec3(front_right.x as f32, 0.0, -front_right.y as f32);
            let left = glm::vec3(left.x as f32, 0.0, -left.y as f32);
            let right = glm::vec3(right.x as f32, 0.0, -right.y as f32);
            let lower = glm::vec3(0.0, -1.0, 0.0);
            let bevel_raise = glm::vec3(0.0, config.bevel_height, 0.0);

            vec!(
                back_left,
                back_right,
                right,
                front_right,
                front_left,
                left,

                right + lower,
                front_right + lower,
                front_left + lower,
                left + lower,

                back_left * config.inner_beveled_hex_scale + bevel_raise,
                back_right * config.inner_beveled_hex_scale + bevel_raise,
                front_left * config.inner_beveled_hex_scale + bevel_raise,
                front_right * config.inner_beveled_hex_scale + bevel_raise,
                left * config.inner_beveled_hex_scale + bevel_raise,
                right * config.inner_beveled_hex_scale + bevel_raise)
        };


        let back_left = 0;
        let back_right = 1;
        let right = 2;
        let front_right = 3;
        let front_left = 4;
        let left = 5;
        let bottom_right = 6;
        let bottom_front_right = 7;
        let bottom_front_left = 8;
        let bottom_left = 9;
        let inner_back_left = 10;
        let inner_back_right = 11;
        let inner_front_left = 12;
        let inner_front_right = 13;
        let inner_left = 14;
        let inner_right = 15;

        let faces = vec!(
            // Bevel ring
            back_left, inner_back_right, back_right,
            back_left, inner_back_left, inner_back_right,
            back_right, inner_right, right,
            back_right, inner_back_right, inner_right,
            right, inner_front_right, front_right,
            right, inner_right, inner_front_right,
            front_right, inner_front_left, front_left,
            front_right, inner_front_right, inner_front_left,
            front_left, inner_left, left,
            front_left, inner_front_left, inner_left,
            left, inner_back_left, back_left,
            left, inner_left, inner_back_left,

            // Inner hexagon.
            inner_back_left, inner_front_right, inner_back_right,
            inner_back_left, inner_front_left, inner_front_right,
            inner_left, inner_front_left, inner_back_left,
            inner_back_right, inner_front_right, inner_right,

            // Front-facing wall.
            left, bottom_left, front_left,
            front_left, bottom_left, bottom_front_left,
            front_left, bottom_front_left, front_right,
            front_right, bottom_front_left, bottom_front_right,
            front_right, bottom_front_right, right,
            right, bottom_front_right, bottom_right);

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
