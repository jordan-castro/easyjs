mod ejr {
    include!("../../bindings.rs");
}
use std::ffi::CString;

/// EJR wrapper
pub struct EJR {
    rt: *mut ejr::EasyJSRHandle,
}

/// Convert a Rust &str into a CString
fn str_to_cstr(val: &str) -> CString {
    CString::new(val).expect("Could not convert val into cString")
}

impl EJR {
    /// Create a new EJR instance
    pub fn new() -> Self {
        let instance = unsafe {
            ejr::ejr_new()
        };

        Self {
            rt: instance
        }
    }

    /// Evaluate a JS script.
    pub fn eval_script(&self, script: &str, script_name: &str) -> i32 {
        // C Conversions
        let script_cstr = str_to_cstr(script);
        let script_name_cstr = str_to_cstr(script_name);

        unsafe {
            ejr::ejr_eval_script(self.rt, script_cstr.as_ptr(), script_name_cstr.as_ptr())
        }
    }

    /// Evaluate a JS module.
    pub fn eval_module(&self, script: &str, script_name: &str) -> i32 {
        // C conversions
        let script_cstr = str_to_cstr(script);
        let script_name_cstr = str_to_cstr(script_name);

        unsafe {
            ejr::ejr_eval_module(self.rt, script_cstr.as_ptr(), script_name_cstr.as_ptr())
        }
    }
}

impl Drop for EJR {
    fn drop(&mut self) {
        unsafe {
            ejr::ejr_free(self.rt);
        }
    }
}