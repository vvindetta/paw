use module_api::Module;
use std::ffi::c_char;

extern "C" fn authenticate(_attempts: i32, _username: *const c_char) -> u32 {
    // real auth logic
    println!("Success!");
    0
}

#[no_mangle]
pub static module: Module = Module { authenticate };
