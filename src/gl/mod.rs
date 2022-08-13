mod bindings;

pub fn view_port(x: i32, y: i32, width: i32, height: i32) -> () {
    unsafe { bindings::glViewport(x, y, width, height) }
}
