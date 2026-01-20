use module_lib::Module;

extern "C" fn authenticate() -> u32 {
    // TODO: real auth logic
    println!("Success!");
    0
}

#[no_mangle]
pub static module: Module = Module {
    name: b"test_module\0".as_ptr() as *const c_char,
    authenticate,
};
