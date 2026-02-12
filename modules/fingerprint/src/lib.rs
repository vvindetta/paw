use module_api::Module;
use std::io::{self, Write};
use std::process::Command;

extern "C" fn authenticate(attempts: i32) -> u32 {
    fn success() -> u32 {
        io::stdout().flush().unwrap();
        print!("\r\x1b[2K[\x1b[32mOK\x1b[0m] Fingerprint\n");
        0
    }
    fn fail() -> u32 {
        io::stdout().flush().unwrap();
        print!("\r\x1b[2K[\x1b[31mFAIL\x1b[0m] Fingerprint\n");
        1
    }

    for counter in 0..attempts {
        print!(
            "\r\x1b[2K[\x1b[33m?\x1b[0m] Fingerprint {}/{}",
            counter, attempts
        );
        io::stdout().flush().unwrap();

        let output = match Command::new("fprintd-verify").output() {
            Ok(out) => out,
            Err(_) => return fail(),
        };

        if output.status.success() {
            return success();
        }
    }
    fail()
}

#[no_mangle]
pub static module: Module = Module {
    name: b"paw_fingerprint\0",
    authenticate,
};
