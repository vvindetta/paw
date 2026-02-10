use std::ffi::CStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};

use libloading::{Library, Symbol};
use module_api::Module as ModuleApi;

struct Module {
    _lib: Library,
    name: String,
    authenticate: extern "C" fn(i32) -> u32,
}

impl Module {
    pub fn load(path: &str) -> Result<Self, libloading::Error> {
        unsafe {
            let lib = Library::new(path)?;

            // TODO
            let module_sym: Symbol<*const ModuleApi> = lib.get(b"module\0")?;
            let module_ref: &ModuleApi = &**module_sym; // deref pointer to struct

            let name = CStr::from_bytes_with_nul(module_ref.name)
                .unwrap()
                .to_string_lossy()
                .into_owned();

            let authenticate = module_ref.authenticate;

            Ok(Module {
                _lib: lib,
                name,
                authenticate,
            })
        }
    }
}

fn get_modules() -> Result<(Vec<Module, i32), Error> {
    let mut modules: Vec<(Module, i32)> = Vec::new();

    let file_handle = File::open("/etc/paw.txt")?;
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
    fn sm_authenticate(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let modules = get_modules();

        for (module, attempts) in modules {
            println!("Starting {} module", module.name);

            let ok = (module.authenticate)(attempts);

            if ok != 0 {
                return PamResultCode::PAM_AUTH_ERR;
            }
        }

        PamResultCode::PAM_SUCCESS
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("set credentials");
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("account management");
        PamResultCode::PAM_SUCCESS
    }
}
