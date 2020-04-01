use crate::{
    app::StatusOr,
    file,
    render::TextureUnit,
};
use gl::{
    self,
    types::*,
};
use glm;
use std::{
    collections::HashMap,
    ffi::CString,
    hash::Hash,
    path::PathBuf,
};

pub trait ShaderUniformKey {
    fn to_cstring(self) -> CString;
}

fn compile_shader(path: &PathBuf, shader_type: GLenum) -> StatusOr<GLuint> {
    let slurped_shader_code = file::util::slurp_file(path)
        .map_err(|err| format!("Error reading shader ({:?}), code: {}", path, err))?;
    let shader_c_str =
        CString::new(slurped_shader_code.as_str())
            .map_err(|err| format!("Couldn't turn shader {:?} into a C string. Reason: {}", path, err))?;
    unsafe {
        let shader_id = gl::CreateShader(shader_type);
        gl::ShaderSource(shader_id, 1, &shader_c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);

        let mut success = GLint::from(gl::FALSE);
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        if success == GLint::from(gl::TRUE) {
            Ok(shader_id)
        } else {
            let mut info_log_len = 0;
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_len);
            let mut info_log = Vec::with_capacity(info_log_len as usize);
            info_log.set_len((info_log_len as usize) - 1);
            gl::GetShaderInfoLog(shader_id, info_log_len, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            let err_string = String::from_utf8(info_log)
                    .map_err(|_err|
                        format!("Shader failed to compile. Explanation was invalid UTF-8. Shader: {:?}", path))?;
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

pub struct ShaderProgram<T> {
    pub program: GLuint,
    uniform_cache: HashMap<T, GLint>,
}

impl <T: std::cmp::Eq + std::hash::Hash> ShaderProgram<T> {
    pub fn from_short_pipeline(vertex_filepath: &PathBuf, fragment_filepath: &PathBuf) -> StatusOr<ShaderProgram<T>> {
        let vertex = compile_shader(vertex_filepath, gl::VERTEX_SHADER)?;
        let fragment = compile_shader(fragment_filepath, gl::FRAGMENT_SHADER)?;
        let shader_program = ShaderProgram {
            program: compile_program(&[vertex, fragment])?,
            uniform_cache: HashMap::new(),
        };
        unsafe {
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }
        Ok(shader_program)
    }

    pub fn from_long_pipeline(vertex_filepath: &PathBuf,
                              geometry_filepath: &PathBuf,
                              fragment_filepath: &PathBuf) -> StatusOr<ShaderProgram<T>> {
        let vertex = compile_shader(vertex_filepath, gl::VERTEX_SHADER)?;
        let geometry = compile_shader(geometry_filepath, gl::GEOMETRY_SHADER)?;
        let fragment = compile_shader(fragment_filepath, gl::FRAGMENT_SHADER)?;
        let shader_program = ShaderProgram {
            program: compile_program(&[vertex, geometry, fragment])?,
            uniform_cache: HashMap::new(),
        };
        unsafe {
            gl::DeleteShader(vertex);
            gl::DeleteShader(geometry);
            gl::DeleteShader(fragment);
        }
        Ok(shader_program)
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn deactivate(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl <T:ShaderUniformKey + Eq + Hash + Copy> ShaderProgram<T> {
    unsafe fn get_uniform_location(&mut self, key: T) -> GLint {
        let program = self.program;
        let cached = self.uniform_cache.entry(key).or_insert_with(|| {
            let c_str = key.to_cstring();
            let c_str_ptr = c_str.as_ptr() as *const GLchar;
            let res = gl::GetUniformLocation(program, c_str_ptr);
            if res < 0 {
                panic!("Uniform: {}, {:?}", res, *c_str_ptr);
            }
            res
        });

        *cached
    }

    pub fn set_bool(&mut self, key: T, b: bool) {
       unsafe {
           gl::Uniform1i(self.get_uniform_location(key), if b { 1 } else { 0 });
       }
    }

    pub fn set_i32(&mut self, key: T, i: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(key), i);
        }
    }

    pub fn set_f32(&mut self, key: T, f: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(key), f);
        }
    }

    pub fn set_vec2(&mut self, key: T, v: glm::Vec2) {
        let value_ptr = &v as *const glm::Vec2 as *const f32;
        unsafe {
            gl::Uniform2fv(self.get_uniform_location(key), 1, value_ptr);
        }
    }

    pub fn set_vec3(&mut self, key: T, v: &glm::Vec3) {
        let value_ptr = v as *const glm::Vec3 as *const f32;
        unsafe {
            gl::Uniform3fv(self.get_uniform_location(key), 1, value_ptr);
        }
    }

    pub fn set_vec4(&mut self, key: T, v: &glm::Vec4) {
        let value_ptr = v as *const glm::Vec4 as *const f32;
        unsafe {
            gl::Uniform4fv(self.get_uniform_location(key), 1, value_ptr);
        }
    }

    pub fn set_mat2(&mut self, key: T, m: &glm::Mat2) {
        let value_ptr = m as *const glm::Mat2 as *const f32;
        unsafe {
            gl::UniformMatrix2fv(self.get_uniform_location(key), 1, gl::FALSE, value_ptr);
        }
    }

    pub fn set_mat3(&mut self, key: T, m: &glm::Mat3) {
        let value_ptr = m as *const glm::Mat3 as *const f32;
        unsafe {
            gl::UniformMatrix3fv(self.get_uniform_location(key), 1, gl::FALSE, value_ptr);
        }
    }

    pub fn set_mat4(&mut self, key: T, m: &glm::Mat4) {
        let value_ptr = m as *const glm::Mat4 as *const f32;
        unsafe {
            gl::UniformMatrix4fv(self.get_uniform_location(key), 1, gl::FALSE, value_ptr);
        }
    }

    pub fn set_texture(&mut self, key: T, texture_unit: TextureUnit) {
        self.set_i32(key, texture_unit.to_texture_uniform());
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
