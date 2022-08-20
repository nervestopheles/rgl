include!("./include.rs");

use std::ffi::CStr;
use std::path::Path;

static TITLE: &str = "project void v1\0";

fn main() {
    let mut gdata = Gdata::new(800, 600);
    gdata.titleptr = TITLE.as_ptr();

    graphics::init(&mut gdata);
    glfw::set_key_callback(gdata.window, Some(utils::exit_key_callback));

    let vertex = {
        let vertex = Shader::new(gl::VERTEX_SHADER);
        let vertex_code = Code::new(Path::new("./src/shaders/glsl/default.vert"));
        if let Err(log) = vertex.load_shader_source_code(vertex_code) {
            println!("Vertex shader error!\n{}", log.1);
            exit();
        }
        vertex
    };

    let fragment = {
        let fragment = Shader::new(gl::FRAGMENT_SHADER);
        let fragment_code = Code::new(Path::new("./src/shaders/glsl/default.frag"));
        if let Err(log) = fragment.load_shader_source_code(fragment_code) {
            println!("Fragment shader error!\n{}", log.1);
            exit();
        }
        fragment
    };

    let _shader_program = {
        let shader_program = gl::create_program();
        if shader_program == 0 {
            println!("Error creating program object.");
            exit();
        }

        gl::attach_shader(shader_program, vertex.id);
        gl::attach_shader(shader_program, fragment.id);

        gl::link_program(shader_program);

        let mut status: gl::int = 0;
        gl::get_programiv(shader_program, gl::LINK_STATUS, &mut status);

        if status == gl::FALSE as gl::int {
            println!("Failed to link shader program!");

            let mut log_len: gl::int = 0;
            gl::get_programiv(shader_program, gl::INFO_LOG_LENGTH, &mut log_len);

            if log_len > 0 {
                let mut log = vec![0i8; log_len as usize];
                let logptr = log.as_mut_ptr();

                let mut written: gl::sizei = 0;

                gl::get_program_info_log(shader_program, log_len, &mut written, logptr);
                let logstr = unsafe { CStr::from_ptr(logptr) }
                    .to_str()
                    .expect("Not valid ptr.");
                println!("Log: {logstr}");
            } else {
                println!("Shader program linking error with no log.");
            }
            exit();
        } else {
            gl::use_program(shader_program);
        }
        shader_program
    };

    let vertices: [gl::float; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let vsize = std::mem::size_of::<gl::float>() * vertices.len();

    let colors: [gl::float; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    let csize = std::mem::size_of::<gl::float>() * colors.len();

    let mut vbos = [0 as gl::uint; 2];
    gl::gen_buffers(2, &mut vbos as *mut u32);

    let vbuf = vbos[0];
    gl::bind_buffer(gl::ARRAY_BUFFER, vbuf);
    gl::buffer_data(
        gl::ARRAY_BUFFER,
        vsize,
        vertices.as_ptr() as *const (),
        gl::STATIC_DRAW,
    );

    let cbuf = vbos[1];
    gl::bind_buffer(gl::ARRAY_BUFFER, cbuf);
    gl::buffer_data(
        gl::ARRAY_BUFFER,
        csize,
        colors.as_ptr() as *const (),
        gl::STATIC_DRAW,
    );

    let mut vao: gl::uint = 0;
    gl::gen_vertex_arrays(1, &mut vao);
    gl::bind_vertex_array(vao);

    gl::enable_vertex_attrib_array(0);
    gl::bind_buffer(gl::ARRAY_BUFFER, vbuf);
    gl::vertex_atrib_pointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

    gl::enable_vertex_attrib_array(1);
    gl::bind_buffer(gl::ARRAY_BUFFER, cbuf);
    gl::vertex_atrib_pointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

    /* main program loop */
    mloop(gdata.window, || -> () {
        gl::clear(gl::COLOR_BUFFER_BIT);
        gl::draw_array(gl::TRIANGLES, 0, 3);
    });
}
