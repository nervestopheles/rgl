mod bindings;

pub use bindings::load;

pub const COLOR_BUFFER_BIT: u32 = bindings::COLOR_BUFFER_BIT;

pub use bindings::GLbitfield as bitfield;
pub use bindings::GLclampf as clampf;
pub use bindings::GLfloat as float;
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

pub fn _gen_buffers(n: sizei, buffers: *mut uint) {
    unsafe { bindings::GenBuffers(n, buffers) }
}
