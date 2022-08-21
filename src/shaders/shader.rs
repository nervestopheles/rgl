use crate::gl;
use crate::shaders::code;
use crate::utils::exit;

use code::Code;
use std::path::Path;

pub struct Shader {
    pub id: gl::uint,
}

impl Shader {
    pub fn new(type_: gl::enum_) -> Self {
        Shader {
            id: gl::create_shader(type_),
        }
    }

    pub fn prepare(shader_type: gl::enum_, path_to_code: &str, compile_error: &str) -> Shader {
        let shader = Shader::new(shader_type);
        let shader_code = Code::new(Path::new(path_to_code));
        if let Err(log) = shader.load_shader_source_code(shader_code) {
            println!(
                "{compile_error}\n{}",
                std::str::from_utf8(&log).expect("Not valid str buffer.")
            );
            exit();
        }
        shader
    }

    pub fn load_shader_source_code(&self, code: Code) -> Result<(), Vec<u8>> {
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
                let mut log = vec![0u8; log_len as usize];
                {
                    let mut written: gl::sizei = 0;
                    gl::get_shader_info_log(
                        self.id,
                        log_len,
                        &mut written,
                        log.as_mut_ptr() as *mut i8,
                    );
                    if written == 0 {
                        return Err("Undefined error on write shader log to buffer"
                            .as_bytes()
                            .to_vec());
                    }
                }
                return Err(log);
            }
            return Err("Shader compilation error with no log.".as_bytes().to_vec());
        }
        Ok(())
    }
}
