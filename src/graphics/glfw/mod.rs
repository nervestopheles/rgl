use std::ffi::c_void;
use std::ffi::CString;
use std::process::exit;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod bindings;

pub const TRUE: i32 = bindings::GLFW_TRUE as i32;
pub const FALSE: i32 = bindings::GLFW_FALSE as i32;

pub const CONTEXT_VERSION_MAJOR: i32 = bindings::GLFW_CONTEXT_VERSION_MAJOR as i32;
pub const CONTEXT_VERSION_MINOR: i32 = bindings::GLFW_CONTEXT_VERSION_MINOR as i32;

pub const OPENGL_PROFILE: i32 = bindings::GLFW_OPENGL_PROFILE as i32;
pub const OPENGL_CORE_PROFILE: i32 = bindings::GLFW_OPENGL_CORE_PROFILE as i32;

pub const RESIZABLE: i32 = bindings::GLFW_RESIZABLE as i32;

pub const PRESS: i32 = bindings::GLFW_PRESS as i32;
pub const KEY_ESCAPE: i32 = bindings::GLFW_KEY_ESCAPE as i32;

pub use bindings::GLFWmonitor as Monitor;
pub use bindings::GLFWwindow as Window;

pub use bindings::GLFWglproc as Glproc;
pub use bindings::GLFWkeyfun as Keyfun;

pub fn init() -> i32 {
    let status = unsafe { bindings::glfwInit() };
    if status != TRUE {
        println!("GLFW not init, status: {status}\nExit.");
        exit(-1);
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
    let ptr = unsafe { bindings::glfwCreateWindow(width, height, title, monitor, share) };
    if ptr == std::ptr::null_mut() as *mut Window {
        println!("Failed to create GLFW window");
        terminate();
        exit(-1);
    }
    ptr
}

pub fn make_context_current(window: *mut Window) -> () {
    unsafe { bindings::glfwMakeContextCurrent(window) }
}

pub fn _get_framebuffer_size(window: *mut Window, width: &mut i32, height: &mut i32) -> () {
    unsafe { bindings::glfwGetFramebufferSize(window, width, height) }
}

pub fn window_should_close(window: *mut Window) -> i32 {
    unsafe { bindings::glfwWindowShouldClose(window) }
}

pub fn set_window_should_clouse(window: *mut Window, value: i32) -> () {
    unsafe { bindings::glfwSetWindowShouldClose(window, value) }
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

pub fn set_key_callback(window: *mut Window, cbfun: Keyfun) -> Keyfun {
    unsafe { bindings::glfwSetKeyCallback(window, cbfun) }
}

pub fn get_proc_address(procname: &'static str) -> *const c_void {
    let procname: CString = CString::new(procname).unwrap();
    let procnameptr = procname.as_ptr();
    unsafe { bindings::glfwGetProcAddress(procnameptr).unwrap() as *const c_void }
}
