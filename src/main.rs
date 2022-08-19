use std::ffi::CStr;
use std::path::Path;

mod graphics;
mod utils;

use graphics::gl;
use graphics::glfw;
use graphics::Gdata;

use utils::exit;
use utils::mloop;

static TITLE: &str = "project void v1\0";

fn main() {
    let nullptr: *const () = std::ptr::null();
    let mut gdata = Gdata::new(800, 600);
    gdata.titleptr = TITLE.as_ptr();
    gdata = graphics::init(gdata);

    glfw::set_key_callback(gdata.window, Some(utils::exit_key_callback));

    let vertices: [gl::float; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let vsize = std::mem::size_of::<gl::float>() * vertices.len();

    let mut vbo: gl::uint = 0;
    gl::gen_buffers(1, &mut vbo);
    gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data(
        gl::ARRAY_BUFFER,
        vsize,
        vertices.as_ptr() as *const (),
        gl::STATIC_DRAW,
    );

    /* shader compilation */
    {
        let vertex_shader: gl::uint = gl::create_shader(gl::VERTEX_SHADER);
        if vertex_shader == 0 {
            println!("Error creating vertex shader.");
            exit();
        }

        /* vertex shader code */
        let vsc = utils::read_file(Path::new("./src/shaders/default.vert"));
        let vscptr = vsc.as_ptr();

        let code = [vscptr];
        let codeptr = code.as_ptr();

        gl::shader_source(
            vertex_shader,
            code.len() as gl::sizei,
            codeptr,
            nullptr as *const i32,
        );
        gl::compile_shader(vertex_shader);

        let mut result: gl::int = 0;
        gl::get_shaderiv(vertex_shader, gl::COMPILE_STATUS, &mut result);
        if result == gl::FALSE as gl::int {
            println!("Vertex shader compilation failed!");

            let mut log_len: gl::int = 0;
            gl::get_shaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut log_len);

            if log_len > 0 {
                let mut log = vec![0i8; log_len as usize];
                let logptr = log.as_mut_ptr();

                let mut written: gl::sizei = 0;

                gl::get_shader_info_log(vertex_shader, log_len, &mut written, logptr);
                let log = unsafe { CStr::from_ptr(logptr) }.to_str().unwrap();
                println!("Shader log:\n{log}");
            }
            exit();
        }
    }

    /* main program loop */
    mloop(gdata.window, || -> () {
        gl::clear(gl::COLOR_BUFFER_BIT);
    });
}
