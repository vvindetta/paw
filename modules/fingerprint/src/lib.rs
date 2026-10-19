use std::ffi::{c_char, CStr};
use std::io::{self, Write};
use std::process::Command;

use module_api::Module;

extern "C" fn authenticate(attempts: i32, username: *const c_char, _config: *const c_char) -> u32 {
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

    let username: String = unsafe {
        CStr::from_ptr(username as *const c_char)
            .to_string_lossy()
            .into_owned()
    };

    for counter in 0..attempts {
        print!(
            "\r\x1b[2K[\x1b[33m?\x1b[0m] Fingerprint {}/{}",
            counter, attempts
        );
        io::stdout().flush().unwrap();

        let output = match Command::new("fprintd-verify").arg(&username).output() {
            Ok(out) => out,
            Err(_) => return fail(),
        };

        if output.status.success() {
            return success();
        }
        print!("\r[\x1b[31mX\x1b[0m]");
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    fail()
}

#[no_mangle]
pub static module: Module = Module { authenticate };
