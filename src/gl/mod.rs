#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod bindings;

pub const COLOR_BUFFER_BIT: u32 = bindings::GL_COLOR_BUFFER_BIT;

pub use bindings::GLbitfield as bitfield;
pub use bindings::GLclampf as clampf;

pub fn view_port(x: i32, y: i32, width: i32, height: i32) -> () {
    unsafe { bindings::glViewport(x, y, width, height) }
}

pub fn clear_color(red: clampf, green: clampf, blue: clampf, alpha: clampf) -> () {
    unsafe { bindings::glClearColor(red, green, blue, alpha) }
}

pub fn clear(mask: bitfield) -> () {
    unsafe { bindings::glClear(mask) }
}
