use std::ffi::c_char;

#[repr(C)]
pub struct Module {
    pub authenticate: extern "C" fn(attempts: i32, username: *const c_char) -> u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    extern "C" fn ok_auth(_attempts: i32, _username: *const c_char) -> u32 {
        0
    }

    extern "C" fn fail_auth(_attempts: i32, _username: *const c_char) -> u32 {
        1
    }

    #[test]
    fn invokes_authenticate_callback() {
        let module = Module {
            authenticate: ok_auth,
        };
        let user = CString::new("alice").unwrap();
        assert_eq!((module.authenticate)(3, user.as_ptr()), 0);
    }

    #[test]
    fn propagates_failure_code() {
        let module = Module {
            authenticate: fail_auth,
        };
        let user = CString::new("bob").unwrap();
        assert_ne!((module.authenticate)(1, user.as_ptr()), 0);
    }
}
