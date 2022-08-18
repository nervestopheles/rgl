fn main() {
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=GL");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    /* let mut sources_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    sources_path.push_str("/src");
    let path = std::path::PathBuf::from(sources_path);

    /* GLFW */
    println!("cargo:rerun-if-changed=/usr/include/GLFW/glfw3.h");
    let glfw_path = path.clone().join("glfw").join("bindings.rs");
    let glfw = bindgen::Builder::default()
        .header("/usr/include/GLFW/glfw3.h")
        .allowlist_var(r#"(GLFW.*)"#)
        .allowlist_type(r#"(GLFW.*)"#)
        .allowlist_function(r#"(glfw.*)"#)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for GLFW.");
    glfw.write_to_file(glfw_path)
        .expect("Couldn't write bindings for GLFW!"); */
}
