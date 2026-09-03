use dlloader::{Loader, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    add: extern "C" fn(a: i32, b: i32) -> i32,
    add_float: extern "C" fn(a: f32, b: f32) -> f32,
    greet: extern "C" fn() -> *const std::ffi::c_char,
}

fn main() {
    let debug_dir = std::env::current_exe()
        .expect("failed to get current exe path")
        .parent()
        .expect("exe has no parent dir")
        .parent()
        .expect("examples has no parent dir")
        .to_path_buf();

    #[cfg(target_os = "windows")]
    let lib = Loader::<PluginApi>::load(debug_dir.join("libdlloader_fixture.dll"))
        .expect("failed to load dlloader_fixture.dll");
    #[cfg(target_os = "linux")]
    let lib = Loader::<PluginApi>::load(debug_dir.join("libdlloader_fixture.so"))
        .expect("failed to load libdlloader_fixture.so");
    #[cfg(target_os = "macos")]
    let lib = Loader::<PluginApi>::load(debug_dir.join("libdlloader_fixture.dylib"))
        .expect("failed to load libdlloader_fixture.dylib");

    println!("add(2, 3) = {}", lib.add(2, 3));

    unsafe {
        let msg = std::ffi::CStr::from_ptr(lib.greet()).to_string_lossy();
        println!("greet() = {msg}");
    }

    println!("add_float(100.3, 10.5) = {}", lib.add_float(100.3, 10.5));
}
