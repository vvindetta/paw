use module_api::Module;
use std::process::Command;

extern "C" fn authenticate(attemps: i32) -> u32 {
    for _ in 0..attemps {
        let output = match Command::new("fprintd-verify").output() {
            Ok(out) => out,
            Err(_) => return 0,
        };

        if output.status.success() {
            return 0;
        }
    1
}

#[no_mangle]
pub static module: Module = Module {
    name: b"paw_fingerprint\0",
    authenticate,
};
