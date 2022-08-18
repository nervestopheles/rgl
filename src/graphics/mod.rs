pub mod gl;
pub mod glfw;

pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

impl Resolution {
    pub fn new(width: i32, height: i32) -> Self {
        Resolution { width, height }
    }
}

pub struct Gdata {
    pub res: Resolution,
    pub titleptr: *const u8,
    pub window: *mut glfw::Window,
}

impl Gdata {
    pub fn new(width: i32, height: i32) -> Self {
        let nullptr: *const () = std::ptr::null();
        Gdata {
            res: Resolution::new(width, height),
            titleptr: nullptr as *const u8,
            window: nullptr as *mut glfw::Window,
        }
    }
}

pub fn init(mut gdata: Gdata) -> Gdata {
    let nullptr: *const () = std::ptr::null();
    let window: *mut glfw::Window;

    glfw::init();

    glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
    glfw::window_hint(glfw::RESIZABLE, glfw::FALSE);
    window = glfw::create_window(
        gdata.res.width,
        gdata.res.height,
        gdata.titleptr,
        nullptr as *mut glfw::Monitor,
        nullptr as *mut glfw::Window,
    );

    glfw::make_context_current(window);
    glfw::set_key_callback(window, Some(exit_key_callback));

    /* load opengl funcs */
    gl::load(glfw::get_proc_address);

    gl::view_port(0, 0, gdata.res.width, gdata.res.height);
    gl::clear_color(0.05, 0.0, 0.1, 0.8);

    gdata.window = window;
    gdata
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
