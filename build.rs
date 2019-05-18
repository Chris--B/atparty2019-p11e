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
}
