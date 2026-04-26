use module_api::Module as ModuleApi;
use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};

use libloading::{Library, Symbol};

use std::error::Error;
use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Module {
    _lib: Library,
    authenticate: extern "C" fn(attempts: i32, username: *const c_char) -> u32,
}

impl Module {
    pub fn load(path: &str) -> Result<Self, libloading::Error> {
        unsafe {
            let lib = Library::new(path)?;

            // TODO
            let module_sym: Symbol<*const ModuleApi> = lib.get(b"module\0")?;
            let module_ref: &ModuleApi = &**module_sym; // deref pointer to struct

            let authenticate = module_ref.authenticate;

            Ok(Module {
                _lib: lib,
                authenticate,
            })
        }
    }
}

fn get_modules() -> Result<Vec<(Module, i32)>, Box<dyn Error>> {
    let mut modules: Vec<(Module, i32)> = Vec::new();

    let file_handle = File::open("/etc/paw.conf")?;
    let reader = BufReader::new(file_handle);

    for line_result in reader.lines() {
        let line = line_result?;

        let mut tokens_iter = line.split_whitespace();
        let path = tokens_iter.next();
        let attempts_number = tokens_iter.next();

        let Some(path) = path else {
            break; // empty line
        };
        let attempts_number_value: i32 = match attempts_number {
            Some(attempts_text) => attempts_text.parse().unwrap_or(3),
            None => 3,
        };

        let module = Module::load(path)?;

        modules.push((module, attempts_number_value));
    }
    return Ok(modules);
}

struct Paw;
pam::pam_hooks!(Paw);

impl PamHooks for Paw {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let Ok(modules) = get_modules() else {
            return PamResultCode::PAM_AUTH_ERR;
        };

        let Ok(username_str) = pamh.get_user(None) else {
            return PamResultCode::PAM_AUTH_ERR;
        };

        let Ok(username_c) = CString::new(username_str) else {
            return PamResultCode::PAM_AUTH_ERR;
        };

        for (module, attempts) in modules.iter() {
            let ok = (module.authenticate)(*attempts, username_c.as_ptr());

            if ok != 0 {
                return PamResultCode::PAM_AUTH_ERR;
            }
        }

        print!("\n");
        PamResultCode::PAM_SUCCESS
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        PamResultCode::PAM_SUCCESS
    }
}
