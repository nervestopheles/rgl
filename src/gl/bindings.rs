use std::os::raw::c_int;

pub type GLint = c_int;
pub type GLsizei = c_int;

extern "C" {
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}
