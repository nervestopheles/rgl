use std::ffi::c_void;

mod bindings;

pub use bindings::load;

pub use bindings::FALSE;

pub use bindings::ARRAY_BUFFER;
pub use bindings::COLOR_BUFFER_BIT;
pub use bindings::STATIC_DRAW;

pub use bindings::COMPILE_STATUS;
pub use bindings::INFO_LOG_LENGTH;

pub use bindings::FRAGMENT_SHADER;
pub use bindings::VERTEX_SHADER;

pub use bindings::GLbitfield as bitfield;
pub use bindings::GLchar as char;
pub use bindings::GLclampf as clampf;
pub use bindings::GLenum as enum_;
pub use bindings::GLfloat as float;
pub use bindings::GLint as int;
pub use bindings::GLsizei as sizei;
pub use bindings::GLuint as uint;

pub fn view_port(x: i32, y: i32, width: i32, height: i32) -> () {
    unsafe { bindings::Viewport(x, y, width, height) }
}

pub fn clear_color(red: clampf, green: clampf, blue: clampf, alpha: clampf) -> () {
    unsafe { bindings::ClearColor(red, green, blue, alpha) }
}

pub fn clear(mask: bitfield) -> () {
    unsafe { bindings::Clear(mask) }
}

pub fn gen_buffers(n: sizei, buffers: *mut uint) {
    unsafe { bindings::GenBuffers(n, buffers) }
}

pub fn bind_buffer(target: uint, buffer: uint) {
    unsafe { bindings::BindBuffer(target, buffer) }
}

pub fn buffer_data(target: uint, size: usize, data: *const (), usage: u32) {
    let data = data as *const c_void;
    unsafe { bindings::BufferData(target, size as isize, data, usage) }
}

pub fn create_shader(type_: enum_) -> uint {
    unsafe { bindings::CreateShader(type_) }
}

pub fn shader_source(shader: uint, count: sizei, string: *const *const char, length: *const int) {
    unsafe { bindings::ShaderSource(shader, count, string, length) }
}

pub fn compile_shader(shader: uint) {
    unsafe { bindings::CompileShader(shader) }
}

pub fn get_shaderiv(shader: uint, pname: enum_, params: *mut int) {
    unsafe { bindings::GetShaderiv(shader, pname, params) }
}

pub fn get_shader_info_log(shader: uint, buf_size: sizei, length: *mut int, info_log: *mut char) {
    unsafe { bindings::GetShaderInfoLog(shader, buf_size, length, info_log) }
}
