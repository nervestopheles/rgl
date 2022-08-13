use std::os::raw::c_char;
use std::os::raw::c_int;

pub const GLFW_TRUE: i32 = 1;
pub const GLFW_FALSE: i32 = 0;

pub const GLFW_CONTEXT_VERSION_MAJOR: i32 = 139266;
pub const GLFW_CONTEXT_VERSION_MINOR: i32 = 139267;

pub const GLFW_OPENGL_PROFILE: i32 = 139272;
pub const GLFW_OPENGL_CORE_PROFILE: i32 = 204801;

pub const GLFW_RESIZABLE: i32 = 131075;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWmonitor {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWwindow {
    _unused: [u8; 0],
}

extern "C" {
    pub fn glfwInit() -> c_int;

    pub fn glfwWindowHint(hint: c_int, value: c_int) -> ();
    pub fn glfwCreateWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: *mut GLFWmonitor,
        share: *mut GLFWwindow,
    ) -> *mut GLFWwindow;

    pub fn glfwGetFramebufferSize(
        window: *mut GLFWwindow,
        width: *mut c_int,
        height: *mut c_int,
    ) -> ();

    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow) -> ();

    pub fn glfwPollEvents() -> ();
    pub fn glfwTerminate() -> ();
}
