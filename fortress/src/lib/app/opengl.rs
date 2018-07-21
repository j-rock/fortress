use gl::{
    self,
    types::GLenum
};

pub fn err_print(file: &'static str, line: u32) {
    unsafe {
        let error = gl::GetError();
        if error != 0 as GLenum {
            println!("Error was {} at {}:{}", error, file, line);
        }
    }
}