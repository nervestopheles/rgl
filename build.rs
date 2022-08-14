use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=GL");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let mut sources_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    sources_path.push_str("/src");
    let path = PathBuf::from(sources_path);

    /* GLFW */
    println!("cargo:rerun-if-changed=/usr/include/GLFW/glfw3.h");
    let glfw_path = path.clone().join("glfw").join("bindings.rs");
    let glfw = bindgen::Builder::default()
        .header("/usr/include/GLFW/glfw3.h")
        .allowlist_var(r#"(GLFW\w*)"#)
        .allowlist_type(r#"(GLFW\w*)"#)
        .allowlist_function(r#"(glfw\w*)"#)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for GLFW.");
    glfw.write_to_file(glfw_path)
        .expect("Couldn't write bindings for GLFW!");

    /* GL */
    println!("cargo:rerun-if-changed=/usr/include/GL/gl.h");
    let gl_path = path.clone().join("gl").join("bindings.rs");
    let gl = bindgen::Builder::default()
        .header("/usr/include/GL/gl.h")
        .allowlist_var(r#"(GL\w*)"#)
        .allowlist_type(r#"(GL\w*)"#)
        .allowlist_function(r#"(gl\w*)"#)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for GL.");
    gl.write_to_file(gl_path)
        .expect("Couldn't write bindings for GL!");
}
