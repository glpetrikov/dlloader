#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn add_float(a: f32, b: f32) -> f32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn greet() -> *const std::ffi::c_char {
    static GREETING: &[u8] = b"hello from dlloader-fixture\0";
    GREETING.as_ptr() as *const std::ffi::c_char
}
