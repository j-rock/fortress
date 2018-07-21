use app::StatusOr;
use file;
use gl::{
    self,
    types::*,
};
use glm;
use std::{
    ffi::CString,
    self,
};

fn compile_shader(path: &str, shader_type: GLenum) -> StatusOr<GLuint> {
    let slurped_shader_code = file::util::slurp_file(path)
        .map_err(|err| format!("Error reading shader ({}), code: {}", path, err))?;
    let shader_c_str =
        CString::new(slurped_shader_code.as_str())
            .map_err(|err| format!("Couldn't turn shader {} into a C string. Reason: {}", path, err))?;
    unsafe {
        let shader_id = gl::CreateShader(shader_type);
        gl::ShaderSource(shader_id, 1, &shader_c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        if success == (gl::TRUE as GLint) {
            Ok(shader_id)
        } else {
            let mut info_log_len = 0;
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_len);
            let mut info_log = Vec::with_capacity(info_log_len as usize);
            info_log.set_len((info_log_len as usize) - 1);
            gl::GetShaderInfoLog(shader_id, info_log_len, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            let err_string = String::from_utf8(info_log)
                    .map_err(|_err|
                        format!("Shader failed to compile. Explanation was invalid UTF-8. Shader: {}", path))?;
            Err(err_string)
        }
    }
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

pub struct ShaderProgram {
    pub program: GLuint
}

impl ShaderProgram {
    pub fn from_short_pipeline(vertex_filepath: &str, fragment_filepath: &str) -> StatusOr<ShaderProgram> {
        let vertex = compile_shader(vertex_filepath, gl::VERTEX_SHADER)?;
        let fragment = compile_shader(fragment_filepath, gl::FRAGMENT_SHADER)?;
        let shader_program = ShaderProgram {
            program: compile_program(&[vertex, fragment])?
        };
        unsafe {
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }
        Ok(shader_program)
    }

    pub fn from_long_pipeline(vertex_filepath: &str,
                              geometry_filepath: &str,
                              fragment_filepath: &str) -> StatusOr<ShaderProgram> {
        let vertex = compile_shader(vertex_filepath, gl::VERTEX_SHADER)?;
        let geometry = compile_shader(geometry_filepath, gl::GEOMETRY_SHADER)?;
        let fragment = compile_shader(fragment_filepath, gl::FRAGMENT_SHADER)?;
        let shader_program = ShaderProgram {
            program: compile_program(&[vertex, geometry, fragment])?
        };
        unsafe {
            gl::DeleteShader(vertex);
            gl::DeleteShader(geometry);
            gl::DeleteShader(fragment);
        }
        Ok(shader_program)
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
    }
}

impl Drop for ShaderProgram {
   fn drop(&mut self) {
       if self.program != 0 {
           unsafe {
               gl::DeleteProgram(self.program);
           }
       }
   }
}

