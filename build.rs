// Useful reference:
//  https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    if cfg!(all(windows, target_env = "msvc")) {
        // Unclear whether we need this one.
        // println!("cargo:rustc-link-lib={}={}", "dylib", "ucrt");
        println!("cargo:rustc-link-lib={}={}", "dylib", "msvcrt");
    } else {
        println!("cargo:rustc-link-lib={}={}", "dylib", "c");
    }

    // Our OpenGL loader is written in C because we're lazy.
    cc::Build::new()
        .define(
            "P11E_DEVBUILD",
            if cfg!(feature = "dev_build") {
                "1"
            } else {
                "0"
            },
        )
        // Windows: Where defining PI is hard
        .define("_USE_MATH_DEFINES","")
        .include("loader")
        .file("loader/ogl.c")
        .file("fft/fft.c")
        .compile("ogl_loader");
}
