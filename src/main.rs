include!("./include.rs");

use std::path::Path;

static TITLE: &str = "project void v1\0";

fn main() {
    let mut gdata = Gdata::new(800, 600);
    gdata.titleptr = TITLE.as_ptr();

    graphics::init(&mut gdata);
    glfw::set_key_callback(gdata.window, Some(utils::exit_key_callback));

    let vertex = Shader::new(gl::VERTEX_SHADER);
    {
        let vertex_code = Code::new(Path::new("./src/shaders/glsl/default.vert"));
        if let Err(log) = vertex.load_shader_source_code(vertex_code) {
            println!("Vertex shader error!\n{}", log.1);
            exit();
        }
    }
    let fragment = Shader::new(gl::FRAGMENT_SHADER);
    {
        let fragment_code = Code::new(Path::new("./src/shaders/glsl/default.frag"));
        if let Err(log) = fragment.load_shader_source_code(fragment_code) {
            println!("Fragment shader error!\n{}", log.1);
            exit();
        }
    }

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

    /* main program loop */
    mloop(gdata.window, || -> () {
        gl::clear(gl::COLOR_BUFFER_BIT);
    });
}
