use app::StatusOr;
use gl::{
    self,
    types::*,
};
use glm;
use render::{
    AttributeProgram,
    FragmentShader,
    GeometryShader,
    VertexShader,
};
use std::{
    ffi::CString,
    self,
};

pub struct ShaderProgram<T> {
    program: GLuint,
    attribute_program: AttributeProgram<T>,
}

impl <T> ShaderProgram<T> {
    pub fn from_short_pipeline(attribute_program: AttributeProgram<T>, vertex: &VertexShader, fragment: &FragmentShader) -> StatusOr<ShaderProgram<T>> {
        Ok(ShaderProgram {
            program: Self::compile_program(&[vertex.shader_id(), fragment.shader_id()])?,
            attribute_program
        })
    }

    pub fn from_long_pipeline(attribute_program: AttributeProgram<T>,
                              vertex: &VertexShader,
                              geometry: &GeometryShader,
                              fragment: &FragmentShader) -> StatusOr<ShaderProgram<T>> {
        Ok(ShaderProgram {
            program: Self::compile_program(&[vertex.shader_id(), geometry.shader_id(), fragment.shader_id()])?,
            attribute_program
        })
    }

    fn compile_program(shaders: &[GLuint]) -> StatusOr<GLuint> {
        unsafe {
            let program_id = gl::CreateProgram();
            for shader in shaders.iter() {
                gl::AttachShader(program_id, *shader);
            }
            gl::LinkProgram(program_id);

            let mut success: GLint = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success as GLboolean == gl::TRUE {
                Ok(program_id)
            } else {
                let mut error_log: Vec<u8> = Vec::with_capacity(512);
                let error_log_ptr = error_log.as_mut_slice().as_mut_ptr() as *mut i8;
                gl::GetProgramInfoLog(program_id, error_log.capacity() as i32, std::ptr::null_mut(), error_log_ptr);
                let err_string =
                    String::from_utf8(error_log)
                        .map_err(|_err|
                            String::from("Program failed to compile. Could not retrieve reason."))?;
                Err(err_string)
            }
        }
    }

    pub fn attributes_mut(&mut self) -> &mut T {
        &mut self.attribute_program.attributes_mut()
    }

    unsafe fn get_uniform_location(&self, name: &'static str) -> GLint {
        let c_str = CString::new(name)
            .map_err(|err| format!("Couldn't uniform name {} into a C string. Reason: {}", name, err)).unwrap();
        let res = gl::GetUniformLocation(self.program, c_str.as_ptr() as *const GLchar);
        if res < 0 {
            panic!("Uniform: {}, {}", res, name);
        }
        res
    }

    pub fn set_bool(&self, name: &'static str, b: bool) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), if b { 1 } else { 0 });
        }
    }

    pub fn set_i32(&self, name: &'static str, i: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), i);
        }
    }

    pub fn set_f32(&self, name: &'static str, f: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), f);
        }
    }

    pub fn set_vec2(&self, name: &'static str, v: &glm::Vec2) {
        unsafe {
            let value_ptr = std::mem::transmute(v);
            gl::Uniform2fv(self.get_uniform_location(name), 1, value_ptr);
        }
    }

    pub fn set_vec3(&self, name: &'static str, v: &glm::Vec3) {
        unsafe {
            let value_ptr = std::mem::transmute(v);
            gl::Uniform3fv(self.get_uniform_location(name), 1, value_ptr);
        }
    }

    pub fn set_vec4(&self, name: &'static str, v: &glm::Vec4) {
        unsafe {
            let value_ptr = std::mem::transmute(v);
            gl::Uniform4fv(self.get_uniform_location(name), 1, value_ptr);
        }
    }

    pub fn set_mat2(&self, name: &'static str, m: &glm::Mat2) {
        unsafe {
            let value_ptr = std::mem::transmute(m);
            gl::UniformMatrix2fv(self.get_uniform_location(name), 1, gl::FALSE, value_ptr);
        }
    }

    pub fn set_mat3(&self, name: &'static str, m: &glm::Mat3) {
        unsafe {
            let value_ptr = std::mem::transmute(m);
            gl::UniformMatrix3fv(self.get_uniform_location(name), 1, gl::FALSE, value_ptr);
        }
    }

    pub fn set_mat4(&self, name: &'static str, m: &glm::Mat4) {
        unsafe {
            let value_ptr = std::mem::transmute(m);
            gl::UniformMatrix4fv(self.get_uniform_location(name), 1, gl::FALSE, value_ptr);
        }
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
        self.attribute_program.activate();
    }

    pub fn deactivate(&self) {
        self.attribute_program.deactivate();
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl <T> Drop for ShaderProgram<T> {
    fn drop(&mut self) {
        if self.program != 0 {
            unsafe {
                gl::DeleteProgram(self.program);
            }
        }
    }
}
