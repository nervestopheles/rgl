use std::ffi::CStr;

use crate::gl;
use crate::shaders::code;

use code::Code;

pub struct Shader {
    id: gl::uint,
}

impl Shader {
    pub fn new(type_: gl::enum_) -> Self {
        Shader {
            id: gl::create_shader(type_),
        }
    }

    pub fn load_shader_code(&self, code: Code) -> Result<(), (Option<Vec<i8>>, &str)> {
        let codeptr = code.string.as_ptr() as *const i8;
        let ptrs = [codeptr];

        gl::shader_source(
            self.id,
            ptrs.len() as gl::sizei,
            ptrs.as_ptr(),
            std::ptr::null() as *const i32,
        );

        gl::compile_shader(self.id);

        let mut result: gl::int = 0;
        gl::get_shaderiv(self.id, gl::COMPILE_STATUS, &mut result);

        if result == gl::FALSE as gl::int {
            let mut log_len: gl::int = 0;
            gl::get_shaderiv(self.id, gl::INFO_LOG_LENGTH, &mut log_len);

            if log_len > 0 {
                let mut log = vec![0i8; log_len as usize];
                let logptr = log.as_mut_ptr();

                let mut written: gl::sizei = 0;

                gl::get_shader_info_log(self.id, log_len, &mut written, logptr);
                let logstr = unsafe { CStr::from_ptr(logptr) }
                    .to_str()
                    .expect("Not valid ptr.");
                return Err((Some(log), logstr));
            }
            return Err((None, "Shader compilation error with no log."));
        }
        Ok(())
    }
}
