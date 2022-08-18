use crate::gl;
use crate::glfw;

pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

pub struct Gdata {
    pub res: Resolution,
    pub window: *mut glfw::Window,
}

pub fn init(width: i32, height: i32, titleptr: *const i8) -> Gdata {
    let nullptr: *const () = std::ptr::null();
    let window: *mut glfw::Window;

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

    /* load opengl funcs */
    gl::load(glfw::get_proc_address);

    gl::view_port(0, 0, width, height);
    gl::clear_color(0.05, 0.0, 0.1, 0.8);

    Gdata {
        window: window,
        res: Resolution { width, height },
    }
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
