use std::ffi::c_char;

#[repr(C)]
pub struct Module {
    pub name: *const c_char,
    pub authenticate: extern "C" fn() -> u32,
}
