use std::process::exit;

mod bindings;

pub const TRUE: i32 = bindings::GLFW_TRUE;
pub const FALSE: i32 = bindings::GLFW_FALSE;

pub const CONTEXT_VERSION_MAJOR: i32 = bindings::GLFW_CONTEXT_VERSION_MAJOR;
pub const CONTEXT_VERSION_MINOR: i32 = bindings::GLFW_CONTEXT_VERSION_MINOR;

pub const OPENGL_PROFILE: i32 = bindings::GLFW_OPENGL_PROFILE;
pub const OPENGL_CORE_PROFILE: i32 = bindings::GLFW_OPENGL_CORE_PROFILE;

pub const RESIZABLE: i32 = bindings::GLFW_RESIZABLE;

pub use bindings::GLFWmonitor as Monitor;
pub use bindings::GLFWwindow as Window;

pub fn init() -> i32 {
    let status = unsafe { bindings::glfwInit() };
    if status != TRUE {
        println!("GLFW not init, status: {status}\nExit.");
        exit(0);
    }
    status
}

pub fn window_hint(hint: i32, value: i32) -> () {
    unsafe { bindings::glfwWindowHint(hint, value) }
}

pub fn create_window(
    width: i32,
    height: i32,
    title: *const i8,
    monitor: *mut Monitor,
    share: *mut Window,
) -> *mut Window {
    // TODO: if return null ptr?
    unsafe { bindings::glfwCreateWindow(width, height, title, monitor, share) }
}

pub fn get_framebuffer_size(window: *mut Window, width: &mut i32, height: &mut i32) -> () {
    unsafe { bindings::glfwGetFramebufferSize(window, width, height) }
}

pub fn window_should_close(window: *mut Window) -> i32 {
    unsafe { bindings::glfwWindowShouldClose(window) }
}

pub fn swap_buffers(window: *mut Window) -> () {
    unsafe { bindings::glfwSwapBuffers(window) }
}

pub fn poll_events() -> () {
    unsafe { bindings::glfwPollEvents() }
}

pub fn terminate() -> () {
    unsafe { bindings::glfwTerminate() }
}
