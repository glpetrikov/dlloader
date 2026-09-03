use dlloader::{Loader, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    add: extern "C" fn(a: i32, b: i32) -> i32,
    add_float: extern "C" fn(a: f32, b: f32) -> f32,
    greet: extern "C" fn() -> *const std::ffi::c_char,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let debug_dir = std::env::current_exe()?
        .parent()
        .ok_or("exe has no parent dir")?
        .parent()
        .ok_or("examples has no parent dir")?
        .to_path_buf();

    let lib = Loader::<PluginApi>::load_with_auto_extension(debug_dir.join("dlloader_fixture"))?;

    println!("add(2, 3) = {}", lib.add(2, 3));
    unsafe {
        let msg = std::ffi::CStr::from_ptr(lib.greet()).to_string_lossy();
        println!("greet() = {msg}");
    }
    println!("add_float(100.3, 10.5) = {}", lib.add_float(100.3, 10.5));

    Ok(())
}
