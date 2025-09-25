mod ejr {
    include!("../../bindings.rs");
}
use std::{collections::HashMap, ffi::{CStr, CString}, os::raw::{c_char, c_void}};

use crate::ffi::ejr::{jsarg_free, C_Callback, C_FileLoaderFn, JSArg};

// TYPES

/// Type for Loader function
type LoaderFn = fn (String) -> String;

/// EJR wrapper
pub struct EJR {
    /// the actual runtime
    rt: *mut ejr::EasyJSRHandle,
    /// The file_loader function
    file_loader_fn: LoaderFn
}

/// Convert a Rust &str into a CString
fn str_to_cstr(val: &str) -> CString {
    CString::new(val).expect("Could not convert val into cString")
}

/// Convert a Vec<mut* ejr::JSArg> ito *mut *mut ejr::JSArg.
/// 
/// Remember to free this later...
fn make_jsarg_array(value: Vec<*mut ejr::JSArg>) -> *mut *mut ejr::JSArg {
    let mut boxed = value.into_boxed_slice();

    let ptr = boxed.as_mut_ptr();

    std::mem::forget(boxed);

    ptr
}

/// Create a JSArg value of Int
pub fn jsarg_int(value: i32) -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_int(value)
    }
} 

/// Create a JSArg value of String
/// Must free if not being used inside of a callback.
pub fn jsarg_string(value: &str) -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_str(str_to_cstr(value).into_raw())
    }
}

/// Create a JSArg value of Double
pub fn jsarg_double(value: f64) -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_double(value)
    }
}

/// Create a JSArg value of Float
pub fn jsarg_float(value: f32) -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_float(value)
    }
}

/// Create a JSArg value of Int64t
pub fn jsarg_int64t(value: i64) -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_int64t(value)
    }
}

/// Create JSArg value of uint32t
pub fn jsarg_uint32t(value: u32) -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_uint32t(value)
    }
}

/// Create JSArg value of C-Array
/// 
/// Must free if not being used in a callback.
pub fn jsarg_carray(values: Vec<*mut ejr::JSArg>) -> *mut ejr::JSArg {
    let c_array = unsafe {ejr::jsarg_carray(values.len())};

    for item in values {
        // Add to array
        unsafe {
            ejr::jsarg_add_value_to_c_array(c_array, item);
        }
    }

    c_array
}

/// Create a JSArg value of Null
pub fn jsarg_null() -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_null()
    }
}

/// Free a JSArg.
pub fn free_jsarg_owned(value: *mut JSArg) {
    unsafe {
        jsarg_free(value);
    }
}

/// Global(Static) function for loading files.
unsafe extern "C" fn global_static_file_loader(file_path: *const c_char, opaque: *mut c_void) -> *mut c_char {
    if file_path.is_null() || opaque.is_null() {
        return std::ptr::null_mut();
    }

    // Get Opaque
    let ejr = unsafe { &mut *(opaque as *mut EJR) };

    // Rust Conversion
    let file_path_rs = unsafe { CStr::from_ptr(file_path).to_string_lossy().into_owned()};

    // Run
    let result = (ejr.file_loader_fn)(file_path_rs);

    // Convert result to *mut c_char
    CString::new(result).unwrap().into_raw()
}

impl EJR {
    /// Create a new EJR instance
    pub fn new() -> Self {
        let instance = unsafe {
            ejr::ejr_new()
        };

        let mut ejr = Self {rt: instance, file_loader_fn: |_| -> String {"".to_string()}};

        unsafe {
            ejr::ejr_set_file_loader(ejr.rt, Some(global_static_file_loader), &mut ejr as *mut _ as *mut c_void);
        }
        ejr
    }

    /// Setup the file loader.
    pub fn set_file_loader(&mut self, func: LoaderFn) {
        self.file_loader_fn = func;
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

    /// Evaluate a Function
    pub fn eval_function(&self, func_name: &str, args: Vec<*mut ejr::JSArg>) -> i32 {
        // C Conversions
        let func_name_cstr = str_to_cstr(func_name);
        let args_len = args.len();
        let args_ptr = make_jsarg_array(args);

        unsafe {
            let result = ejr::ejr_eval_function(self.rt, func_name_cstr.as_ptr(), args_ptr, args_len);
            // Free
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(args_ptr, args_len));
            result
        }
    }

    /// Convert a JSValue into a String.
    pub fn val_to_string(&self, value_id: i32) -> String {
        unsafe {
            let result = ejr::ejr_val_to_string(self.rt, value_id);
            CString::from_raw(result).to_string_lossy().to_string()
        }
    }

    /// Evalute a class/object/prototype function
    pub fn evall_class_function(&self, value_id: i32, func_name: &str, args: Vec<*mut ejr::JSArg>) -> i32 {
        // C Conversions
        let func_name_cstr = str_to_cstr(func_name);
        let args_len = args.len();
        let args_ptr = make_jsarg_array(args);

        unsafe {
            let result = ejr::ejr_eval_class_function(self.rt, value_id, func_name_cstr.as_ptr(), args_ptr, args_len);
            // Free
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(args_ptr, args_len));
            result
        }
    }
    // int ejr_eval_class_function(EasyJSRHandle* handle, int value_id, const char* fn_name, JSArg** args, size_t arg_count);

    /// Get a JSArg from a JSValue
    pub fn jsarg_from_jsvalue(&self, value: i32) -> *mut JSArg {
        unsafe {
            ejr::jsarg_from_jsvalue(self.rt, value)
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