use module_api::Module;

extern "C" fn authenticate() -> u32 {
    // real auth logic
    println!("Success!");
    0
}

#[no_mangle]
pub static module: Module = Module {
    name: b"test_module\0",
    authenticate,
};
