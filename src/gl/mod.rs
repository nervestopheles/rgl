#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod bindings;

pub fn view_port(x: i32, y: i32, width: i32, height: i32) -> () {
    unsafe { bindings::glViewport(x, y, width, height) }
}
