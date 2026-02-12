use module_api::Module;

extern "C" fn authenticate(_attempts: i32) -> u32 {
    // real auth logic
    println!("Success!");
    0
}

#[no_mangle]
pub static module: Module = Module {
    name: b"paw_test\0",
    authenticate,
};
