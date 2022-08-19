use crate::glfw;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

/* 30 FPS */
const FRAMETIME: u64 = 16667 * 2;

pub fn mloop(window: *mut glfw::Window, foo: fn()) {
    let frametime = Duration::from_micros(FRAMETIME);
    let mut now: Instant;
    let mut delta: Duration;

    loop {
        if glfw::window_should_close(window) != 0 {
            exit()
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

pub fn exit() {
    glfw::terminate();
    println!("Exit.");
    std::process::exit(0);
}

pub extern "C" fn exit_key_callback(
    window: *mut glfw::Window,
    key: i32,
    _scancode: i32,
    action: i32,
    _mode: i32,
) {
    if key == glfw::KEY_ESCAPE && action == glfw::PRESS {
        glfw::set_window_should_close(window, glfw::TRUE)
    }
}

/* reading full file in cstring */
pub fn read_file(path: &Path) -> CString {
    let mut file = File::open(path).expect("Cant open the file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Read file error.");
    CString::new(data).expect("Cstring new() failed.")
}
