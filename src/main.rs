mod graphics;
mod utils;

use graphics::gl;
use graphics::glfw;
use graphics::Gdata;

use utils::mloop;

static TITLE: &str = "project void v1\0";

fn main() {
    let mut gdata = Gdata::new(800, 600);

    gdata.titleptr = TITLE.as_ptr();
    gdata = graphics::init(gdata);

    glfw::set_key_callback(gdata.window, Some(utils::exit_key_callback));

    let _vertices: [gl::float; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    /* main program loop */
    mloop(gdata.window, || -> () {
        gl::clear(gl::COLOR_BUFFER_BIT);
    });
}
