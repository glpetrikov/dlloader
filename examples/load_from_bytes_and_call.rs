use dlloader::{Loader, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    add: extern "C" fn(a: i32, b: i32) -> i32,
    add_float: extern "C" fn(a: f32, b: f32) -> f32,
    greet: extern "C" fn() -> *const std::ffi::c_char,
}

fn main() {
    #[cfg(target_os = "windows")]
    let lib_bytes = include_bytes!("../target/debug/dlloader_fixture.dll");
    #[cfg(target_os = "linux")]
    let lib_bytes = include_bytes!("../target/debug/libdlloader_fixture.so");
    #[cfg(target_os = "macos")]
    let lib_bytes = include_bytes!("../target/debug/libdlloader_fixture.dylib");

    let lib = Loader::<PluginApi>::load_from_bytes(lib_bytes, "Library")
        .expect("failed to load from bytes libdlloader_fixture");

    println!("add(2, 3) = {}", lib.add(2, 3));

    unsafe {
        let msg = std::ffi::CStr::from_ptr(lib.greet()).to_string_lossy();
        println!("greet() = {msg}");
    }

    println!("add_float(100.3, 10.5) = {}", lib.add_float(100.3, 10.5));
}
