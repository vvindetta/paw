use std::collections::HashMap;
use std::ffi::CStr;

use once_cell::sync::Lazy;

use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};

use libloading::{Library, Symbol};
use module_api::Module as ModuleApi;

struct Module {
    _lib: Library,
    name: String,
    authenticate: extern "C" fn() -> u32,
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

// TODO
static MODULES: Lazy<HashMap<String, Module>> = Lazy::new(|| {
    let module = Module::load("/etc/paw/modules/password.so").expect("Failed to load module");
    HashMap::from([(module.name.to_string(), module)])
});

struct Paw;
pam::pam_hooks!(Paw);

impl PamHooks for Paw {
    fn sm_authenticate(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        for (name, module) in MODULES.iter() {
            println!("Starting {name} module");

            let ok = (module.authenticate)();
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
