use module_api::Module;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

extern "C" fn authenticate(attempts: i32) -> u32 {
    fn success() -> u32 {
        io::stdout().flush().unwrap();
        print!("\r\x1b[2K[\x1b[32mOK\x1b[0m] Password\n");
        0
    }
    fn fail(text: String) -> u32 {
        io::stdout().flush().unwrap();
        print!("\r\x1b[2K[\x1b[31mFAIL\x1b[0m] {}\n", text);
        1
    }

    let path_to_password_hash = "/etc/paw_password_hash.txt";

    let file_handle = match File::open(path_to_password_hash) {
        Ok(file_handle) => file_handle,
        Err(_) => {
            return fail(format!(
                "Password: failed to open {}",
                path_to_password_hash
            ))
        }
    };
    let reader = BufReader::new(file_handle);

    let password_hash = match reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(_)) => {
            return fail(format!(
                "Password: failed to read {}",
                path_to_password_hash
            ))
        }
        None => return fail(format!("Password: file is empty {}", path_to_password_hash)),
    };
    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(hash) => hash,
        Err(_) => return fail("Password: parse hash".to_string()),
    };

    print!("\r\x1b[2K"); // clear current line
    print!("\n"); // create UI line
    print!("\x1b[1A"); // go back to UI line
    print!("\x1b[s"); // save cursor at UI line
    io::stdout().flush().unwrap();

    for counter in 0..attempts {
        // always go to anchored UI line
        print!("\x1b[u\r\x1b[2K");
        print!("[\x1b[33m?\x1b[0m] Password {}/{}: ", counter, attempts);
        io::stdout().flush().unwrap();

        // save position right after ": " (still on the UI line)
        print!("\x1b[s");
        io::stdout().flush().unwrap();

        let password = match rpassword::read_password() {
            Ok(password) => password,
            Err(_) => return fail("Password: failed read password".to_string()),
        };

        // restore to prompt position (UI line), clear tail
        print!("\x1b[u\x1b[0K");
        io::stdout().flush().unwrap();

        // show checking (blue) on UI line
        print!("\x1b[u\r\x1b[2K");
        print!("[\x1b[34m?\x1b[0m] Password {}/{}", counter, attempts);
        io::stdout().flush().unwrap();

        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            return success();
        }
    }
    fail("Password".to_string())
}

#[no_mangle]
pub static module: Module = Module {
    name: b"paw_password\0",
    authenticate,
};
