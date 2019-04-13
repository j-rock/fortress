use gl::{
    self,
    types::*,
};
use std;

pub enum NumComponents {
    // Like int, float
    S1,
    // Like glm::Vec2,
    S2,
    // Like glm::Vec3,
    S3,
    // Like glm::Vec4
    S4
}

impl NumComponents {
    fn into_gl_size(self) -> GLint {
        match self {
            NumComponents::S1 => 1,
            NumComponents::S2 => 2,
            NumComponents::S3 => 3,
            NumComponents::S4 => 4,
        }
    }
}

// Add types here as necessary.
pub enum ComponentType {
    Float, UnsignedInt
}

impl ComponentType {
    fn into_gl_enum(self) -> GLenum {
        match self {
            ComponentType::Float => gl::FLOAT,
            ComponentType::UnsignedInt => gl::UNSIGNED_INT,
        }
    }
}

pub trait KnownComponent {
    fn component() -> (NumComponents, ComponentType);
}

pub struct AttributeProgramBuilder {
    vao: GLuint,
    num_attributes: GLuint,
}

impl AttributeProgramBuilder {
    pub fn add_attribute<T: KnownComponent>(&mut self) -> Attribute<T> {
        self.num_attributes += 1;
        Attribute::<T>::new(self.num_attributes - 1)
    }

    pub fn build(self) -> AttributeProgram {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        let attr_prgm = AttributeProgram {
            vao: self.vao,
        };
        attr_prgm.deactivate();
        attr_prgm
    }
}

pub struct AttributeProgram {
    vao: GLuint,
}

impl AttributeProgram {
    pub fn builder_with_offset(preexisting_num_attributes: GLuint) -> AttributeProgramBuilder {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        AttributeProgramBuilder {
            vao,
            num_attributes: preexisting_num_attributes,
        }
    }

    pub fn builder() -> AttributeProgramBuilder {
        Self::builder_with_offset(0)
    }

    pub fn activate(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn deactivate(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for AttributeProgram {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
        }
    }
}

pub struct Attribute<T> {
    vbo: GLuint,
    pub data: Vec<T>,
}

impl<T> Attribute<T> {
    fn new<U: KnownComponent>(vertex_attrib_array_index: GLuint) -> Attribute<U> {
        let mut vbo: GLuint = 0;
        let data = vec!();
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, data.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
        }

        let data_element_byte_size = std::mem::size_of::<U>() as i32;
        let (num_comp, comp_type) = U::component();
        unsafe {
            gl::EnableVertexAttribArray(vertex_attrib_array_index);
            gl::VertexAttribPointer(vertex_attrib_array_index, num_comp.into_gl_size(), comp_type.into_gl_enum(), gl::FALSE, data_element_byte_size, std::ptr::null());
            gl::VertexAttribDivisor(vertex_attrib_array_index, 1);
        }

        Attribute {
            vbo,
            data,
        }
    }

    pub fn prepare_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            let data_size = self.data.len() as isize * Self::data_element_byte_size();
            gl::BufferData(gl::ARRAY_BUFFER, data_size, self.data.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
        }
    }

    fn data_element_byte_size() -> isize {
        std::mem::size_of::<T>() as isize
    }
}

impl<T> Drop for Attribute<T> {
   fn drop(&mut self) {
       if self.vbo != 0 {
           unsafe {
               gl::DeleteBuffers(1, &self.vbo);
           }
       }
   }
}
