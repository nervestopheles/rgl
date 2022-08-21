static TITLE: &str = "project void v1\0";

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
    pub fn init(width: i32, height: i32) -> Self {
        let nullptr: *const () = std::ptr::null();

        let windowptr: *mut glfw::Window;
        let titleptr = TITLE.as_ptr();

        glfw::init();

        glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
        glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 3);
        glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint(glfw::RESIZABLE, glfw::FALSE);

        windowptr = glfw::create_window(
            width,
            height,
            titleptr,
            nullptr as *mut glfw::Monitor,
            nullptr as *mut glfw::Window,
        );
        glfw::make_context_current(windowptr);

        /* load opengl funcs */
        gl::load(glfw::get_proc_address);
        gl::clear_color(0.05, 0.0, 0.1, 0.8);

        Gdata {
            res: Resolution::new(width, height),
            titleptr: TITLE.as_ptr(),
            window: windowptr,
        }
    }
}
