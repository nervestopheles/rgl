mod graphics;
use graphics::gl;
use graphics::glfw;

use graphics::Gdata;

use std::ffi::CString;
use std::thread::sleep;
use std::time::{Duration, Instant};

const TITLE: &str = "project void v1";
const FRAMETIME: u64 = 16667 * 2; /* 30 FPS */

fn main() {
    let title: CString = CString::new(TITLE).unwrap();
    let mut gdata = Gdata::new(800, 600);
    gdata.titleptr = title.as_ptr();

    gdata = graphics::init(gdata);

    let _vertices: [gl::float; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    mloop(gdata.window, || -> () {
        gl::clear(gl::COLOR_BUFFER_BIT);
    });

    glfw::terminate();
    println!("Exit.")
}

/* main program loop */
fn mloop(window: *mut glfw::Window, foo: fn()) {
    let frametime = Duration::from_micros(FRAMETIME);
    let mut now: Instant;
    let mut delta: Duration;

    loop {
        if glfw::window_should_close(window) != 0 {
            break;
        }

        now = Instant::now();
        glfw::poll_events();

        foo();

        glfw::swap_buffers(window);
        delta = now.elapsed();

        if frametime > delta {
            sleep(frametime - delta);
        }
    }
}
