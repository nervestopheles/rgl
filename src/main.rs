mod gl;
mod glfw;

use std::ffi::CString;
use std::thread::sleep;
use std::time::{Duration, Instant};

const TITLE: &str = "project void v1";

fn main() {
    let nullptr: *const () = std::ptr::null();

    let title: CString = CString::new(TITLE).unwrap();
    let titleptr = title.as_ptr();

    let mut width: i32 = 800;
    let mut height: i32 = 600;
    let window: *mut glfw::Window;

    let frametime = Duration::from_micros(16667);
    let mut now: Instant;
    let mut delta: Duration;

    glfw::init();

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
    glfw::window_hint(glfw::RESIZABLE, glfw::FALSE);
    window = glfw::create_window(
        width,
        height,
        titleptr,
        nullptr as *mut glfw::Monitor,
        nullptr as *mut glfw::Window,
    );
    glfw::make_context_current(window);

    glfw::set_key_callback(window, Some(exit_key_callback));

    gl::clear_color(0.05, 0.0, 0.1, 0.8);

    glfw::get_framebuffer_size(window, &mut width, &mut height);
    gl::view_port(0, 0, width, height);

    /* loop */
    while glfw::window_should_close(window) == 0 {
        now = Instant::now();
        glfw::poll_events();
        {
            gl::clear(gl::COLOR_BUFFER_BIT);
        }
        glfw::swap_buffers(window);
        delta = now.elapsed();
        if frametime > delta {
            sleep(frametime - delta);
        }
    }

    glfw::terminate();
    println!("Exit.")
}

extern "C" fn exit_key_callback(
    window: *mut glfw::Window,
    key: i32,
    _scancode: i32,
    action: i32,
    _mode: i32,
) {
    if key == glfw::KEY_ESCAPE && action == glfw::PRESS {
        glfw::set_window_should_clouse(window, glfw::TRUE)
    }
}
