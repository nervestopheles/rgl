fn main() {
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=GL");
}
