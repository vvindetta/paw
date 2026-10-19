use module_api::Module as ModuleApi;
use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};

use libloading::{Library, Symbol};

use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use toml::Value;

struct Module {
    _lib: Library,
    authenticate:
        extern "C" fn(attempts: i32, username: *const c_char, config: *const c_char) -> u32,
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

struct Paw;
pam::pam_hooks!(Paw);

impl PamHooks for Paw {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let Ok(username_str) = pamh.get_user(None) else {
            return PamResultCode::PAM_AUTH_ERR;
        };
        let Ok(username_c) = CString::new(username_str) else {
            return PamResultCode::PAM_AUTH_ERR;
        };

        let Ok(config_text) = std::fs::read_to_string("/etc/paw.toml") else {
            return PamResultCode::PAM_AUTH_ERR;
        };
        let Ok(config) = toml::from_str::<toml::Table>(&config_text) else {
            return PamResultCode::PAM_AUTH_ERR;
        };

        let Ok(modules) = config["modules"].as_table().ok_or("Invalid modules") else {
            return PamResultCode::PAM_AUTH_ERR;
        };
        let Some(policy) = config.get("policy").and_then(Value::as_array) else {
            return PamResultCode::PAM_AUTH_ERR;
        };

        'groups: for group in policy {
            let Some(group) = group.as_array() else {
                return PamResultCode::PAM_AUTH_ERR;
            };

            if group.is_empty() == true {
                return PamResultCode::PAM_AUTH_ERR;
            };

            for module_name in group {
                let Some(module_name) = module_name.as_str() else {
                    return PamResultCode::PAM_AUTH_ERR;
                };
                let Some(module) = modules.get(module_name) else {
                    return PamResultCode::PAM_AUTH_ERR;
                };

                let attempts = module
                    .get("attempts")
                    .and_then(Value::as_integer)
                    .unwrap_or(3);

                let Ok(attempts) = i32::try_from(attempts) else {
                    return PamResultCode::PAM_AUTH_ERR;
                };

                let Some(path) = module.get("path").and_then(Value::as_str) else {
                    return PamResultCode::PAM_AUTH_ERR;
                };

                let default_config = Value::Table(toml::Table::new());
                let module_config = module.get("config").unwrap_or(&default_config);

                let Ok(module_config_c) = CString::new(module_config.as_str()) else {
                    return PamResultCode::PAM_AUTH_ERR;
                };

                let Ok(module) = Module::load(path) else {
                    return PamResultCode::PAM_AUTH_ERR;
                };

                if (module.authenticate)(attempts, username_c.as_ptr(), module_config_c.as_ptr())
                    != 0
                {
                    continue 'groups;
                }
            }
            return PamResultCode::PAM_SUCCESS;
        }
        return PamResultCode::PAM_AUTH_ERR;
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        PamResultCode::PAM_SUCCESS
    }
}
