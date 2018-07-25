use gl;

pub fn err_print(file: &'static str, line: u32) {
    unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("Error was {} at {}:{}", error, file, line);
        }
    }
}