mod gl;
mod glfw;

use std::ffi::CString;
use std::thread::sleep;
use std::time::{Duration, Instant};

const TITLE: &str = "obj v1";

fn main() {
    let nullptr: *const () = std::ptr::null();

    let title: CString = CString::new(TITLE).unwrap();
    let titleptr = title.as_ptr();

    let mut width: i32 = 800;
    let mut height: i32 = 600;

    glfw::init();

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
    glfw::window_hint(glfw::RESIZABLE, glfw::FALSE);
    let window: *mut glfw::Window = glfw::create_window(
        width,
        height,
        titleptr,
        nullptr as *mut glfw::Monitor,
        nullptr as *mut glfw::Window,
    );

    glfw::get_framebuffer_size(window, &mut width, &mut height);
    gl::view_port(0, 0, width, height);

    let frametime = Duration::from_micros(16667);
    let mut now: Instant;
    let mut delta: Duration;

    /* loop */
    while glfw::window_should_close(window) == 0 {
        now = Instant::now();
        glfw::poll_events();
        /* */
        glfw::swap_buffers(window);
        delta = now.elapsed();
        if frametime > delta {
            sleep(frametime - delta);
        }
    }

    glfw::terminate();
    println!("Exit.")
}
