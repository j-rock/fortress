use crate::render::AttributeProgram;
use gl::{
    self,
    types::*,
};
use glm;

pub struct InstancedMesh {
    vbo: GLuint,
    ebo: GLuint,
    faces: Vec<GLuint>,
    _vertices: Vec<glm::Vec3>,
}

impl InstancedMesh {
    pub fn from_geometry(vertices: Vec<glm::Vec3>, faces: Vec<GLuint>, attribute_program: &AttributeProgram) -> InstancedMesh {
        attribute_program.activate();

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

        attribute_program.deactivate();

        InstancedMesh {
            vbo,
            ebo,
            faces,
            _vertices: vertices,
        }
    }

    // Requires parent AttributeProgram to be active.
    pub fn draw(&self, num_instances: usize) {
        unsafe {
            gl::DrawElementsInstanced(gl::TRIANGLES, self.faces.len() as GLsizei, gl::UNSIGNED_INT, std::ptr::null(), num_instances as GLsizei);
        }
    }
}

impl Drop for InstancedMesh {
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
