use std::ffi::c_char;

#[repr(C)]
pub struct Module {
    pub authenticate: extern "C" fn(attempts: i32, username: *const c_char) -> u32,
}
