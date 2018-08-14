use app::StatusOr;
use file;
use gl::{
    self,
    types::*,
};
use std::{
    ffi::CString,
    path::PathBuf,
    self,
    marker::PhantomData,
};

trait ShaderType {
    fn gl_enum() -> GLenum;
}

struct ShaderVertex;
struct ShaderGeometry;
struct ShaderFragment;

impl ShaderType for ShaderVertex {
    fn gl_enum() -> GLenum {
        gl::VERTEX_SHADER
    }
}

impl ShaderType for ShaderGeometry {
    fn gl_enum() -> GLenum {
        gl::GEOMETRY_SHADER
    }
}

impl ShaderType for ShaderFragment {
    fn gl_enum() -> GLenum {
        gl::FRAGMENT_SHADER
    }
}

pub struct Shader<T> {
    shader_id: GLuint,
    phantom: PhantomData<T>,
}

pub type VertexShader = Shader<ShaderVertex>;
pub type GeometryShader = Shader<ShaderGeometry>;
pub type FragmentShader = Shader<ShaderFragment>;

impl <T: ShaderType> Shader<T> {
    pub fn new(path: &PathBuf) -> StatusOr<Shader<T>> {
        let slurped_shader_code = file::util::slurp_file(path)
            .map_err(|err| format!("Error reading shader ({:?}), code: {}", path, err))?;
        let shader_c_str =
            CString::new(slurped_shader_code.as_str())
                .map_err(|err| format!("Couldn't turn shader {:?} into a C string. Reason: {}", path, err))?;
        unsafe {
            let shader_id = gl::CreateShader(T::gl_enum());
            gl::ShaderSource(shader_id, 1, &shader_c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success == (gl::TRUE as GLint) {
                return Ok(Shader {
                    shader_id,
                    phantom: PhantomData,
                });
            }

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

    pub fn shader_id(&self) -> GLuint {
        self.shader_id
    }
}

impl <T> Drop for Shader<T> {
    fn drop(&mut self) {
        if self.shader_id != 0 {
            unsafe {
                gl::DeleteShader(self.shader_id);
            }
        }
    }
}
