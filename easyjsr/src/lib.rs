#![allow(warnings)]

mod ejr {
    include!("../bindings.rs");
}
use std::{any::Any, collections::HashMap, ffi::{CStr, CString}, os::raw::{c_char, c_void}, ptr::{slice_from_raw_parts, slice_from_raw_parts_mut}};

// TYPES

// pub enum JSA
// pub const JSArgType_JSARG_TYPE_INT: JSArgType = 0;
// pub const JSArgType_JSARG_TYPE_DOUBLE: JSArgType = 1;
// pub const JSArgType_JSARG_TYPE_STRING: JSArgType = 2;
// pub const JSArgType_JSARG_TYPE_FLOAT: JSArgType = 3;
// pub const JSArgType_JSARG_TYPE_BOOL: JSArgType = 4;
// pub const JSArgType_JSARG_TYPE_INT64_T: JSArgType = 5;
// pub const JSArgType_JSARG_TYPE_UINT32_T: JSArgType = 6;
// pub const JSArgType_JSARG_TYPE_C_ARRAY: JSArgType = 7;
// pub const JSArgType_JSARG_TYPE_NULL: JSArgType = 8;

/// Type for Loader function
type LoaderFn = fn (String) -> String;
pub type JSArg = *mut ejr::JSArg;
pub type Opaque = *mut c_void;
pub type JSArgResult = Option<JSArg>;
/// Type for Rust callbacks
type RustCallbackFn = fn (Vec<JSArg>, Opaque) -> Option<*mut ejr::JSArg>;

/// EJR wrapper
pub struct EJR {
    /// the actual runtime
    rt: *mut ejr::EasyJSRHandle,
    /// The file_loader function
    file_loader_fn: LoaderFn,
    /// id => rust callback fn
    cb_fns: HashMap<u32, RustCallbackFn>,
    /// id => user data for functions.
    user_data: HashMap<u32, Box<dyn Any>>,
    /// <OpaqueObject>[]
    opaque_objects: Vec<*mut c_void>, 
    /// Next user data id... 
    next_user_data_id: u32,
    /// Next callback id
    next_cb_fn_id: u32,
}

/// Opaque Object
pub struct OpaqueObject {
    /// Pointer to runtime
    ejr: *mut EJR,
    /// Callback id
    cb_id: u32,
    /// id of Custom user defined data
    user_data: u32
}

/// JSMethod wrapper
pub struct JSMethod {
    /// method Name
    name: String,
    /// Actual callback
    method: RustCallbackFn,
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
pub fn free_jsarg_owned(value: *mut ejr::JSArg) {
    unsafe {
        ejr::jsarg_free(value);
    }
}

/// Derefernce a JSArg
pub fn derefernce_jsarg(value: &JSArg) -> ejr::JSArg {
    let jsarg_ptr: JSArg = *value;
    unsafe {
        let jsarg: ejr::JSArg = *jsarg_ptr;
        jsarg
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

/// Global(static) function for calling a callback
unsafe extern "C" fn global_static_callback_wrappper(args: *mut *mut ejr::JSArg, argc: usize, opaque: *mut c_void) -> *mut ejr::JSArg {
    // Get EJR
    let oo = unsafe { &mut *(opaque as *mut OpaqueObject)};
    let ejr: &mut EJR = unsafe { &mut *oo.ejr };

    // Call fn
    let cb_fn = ejr.cb_fns.get(&oo.cb_id);
    if cb_fn == None {
        return jsarg_null();
    }

    let cb_fn = cb_fn.unwrap();

    // RS Conversion
    let args_rs = unsafe {std::slice::from_raw_parts(args, argc) };

    let result = (cb_fn)(args_rs.to_vec(), opaque);

    if result == None {
        return jsarg_null();
    }

    result.unwrap()
}

impl EJR {
    /// Create a new EJR instance
    pub fn new() -> Self {
        let instance = unsafe {
            ejr::ejr_new()
        };

        let mut ejr = Self {
            rt: instance, 
            file_loader_fn: |_| -> String {"".to_string()},
            cb_fns: HashMap::new(),
            user_data: HashMap::new(),
            next_user_data_id: 0,
            next_cb_fn_id: 0,
            opaque_objects: Vec::new()
        };

        unsafe {
            ejr::ejr_set_file_loader(ejr.rt, Some(global_static_file_loader), &mut ejr as *mut _ as *mut c_void);
        }
        ejr
    }

    // Rust first methods
    
    /// Get from user_data
    pub fn get_user_data<T: 'static>(&self, id: u32) -> Option<&T> {
        self.user_data.get(&id)?.downcast_ref::<T>()
    }

    /// Get mut from user_data
    pub fn get_user_data_mut<T: 'static>(&mut self, id: u32) -> Option<&mut T> {
        self.user_data.get_mut(&id)?.downcast_mut::<T>()
    }

    fn insert_user_data<T: 'static>(&mut self, data: T) -> u32 {
        let id = self.next_user_data_id;
        self.user_data.insert(id, Box::new(data));
        self.next_user_data_id += 1;

        id
    }

    fn insert_cb(&mut self, cb: RustCallbackFn) -> u32 {
        let id = self.next_cb_fn_id;
        self.cb_fns.insert(id, cb);
        self.next_cb_fn_id += 1;
        
        id
    }

    fn add_oo(&mut self, oo: *mut c_void) {
        self.opaque_objects.push(oo);
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

    /// Get a property from a object.
    pub fn get_property_from(&self, value_id: i32, property_name: &str) -> i32 {
        // C conversions
        let property_name_cstr = str_to_cstr(property_name);

        unsafe {
            ejr::ejr_get_property_from(self.rt, value_id, property_name_cstr.as_ptr())
        }
    }

    /// Get a property from globalThis scope
    pub fn get_property_from_global(&self, property_name: &str) -> i32 {
        // C conversions
        let property_name_cstr = str_to_cstr(property_name);

        unsafe {
            ejr::ejr_get_from_global(self.rt, property_name_cstr.as_ptr())
        }
    }

    /// Register a Rust callback in JS.
    pub fn register_callback(&mut self, func_name: &str, cb: RustCallbackFn, opaque: Box<dyn Any>) {
        // C conversion
        let func_name_cstr = str_to_cstr(func_name);

        // Save CB and opaque data
        let cb_id = self.insert_cb(cb);
        let user_data_id = self.insert_user_data(opaque);

        // Create opaque object
        let oo = Box::new(OpaqueObject{
            ejr: self as *mut _,
            cb_id: cb_id,
            user_data: user_data_id
        });

        // Save opaque object
        let oo_ptr = Box::into_raw(oo)as *mut c_void;

        self.add_oo(oo_ptr);

        unsafe {
            ejr::ejr_register_callback(
                self.rt, 
                func_name_cstr.as_ptr(), 
                Some(global_static_callback_wrappper), 
                oo_ptr
            );
        }
    }

    /// Register a JS Module with Rust callbacks
    pub fn register_module(&mut self, module_name: &str, methods: Vec<JSMethod>, user_data: Box<dyn Any>) {
        // C conversions
        let module_name_cstr = str_to_cstr(module_name);
        let mut methods_ptrs = vec![];
        let mut name_cstrs = vec![];
        // Save user_data
        let user_data_id = self.insert_user_data(user_data);
        let ejr_ptr = self as *mut _;

        for m in methods.iter() {
            let name_cstr = str_to_cstr(m.name.as_str());
            // save in name_cstrs to not free yet
            name_cstrs.push(name_cstr);
            // Save callback
            let cb_id = self.insert_cb(m.method);

            let oo = Box::new(OpaqueObject{
                ejr: ejr_ptr,
                cb_id: cb_id,
                user_data: user_data_id
            });
            let oo_ptr = Box::into_raw(oo) as *mut c_void;
            self.add_oo(oo_ptr);

            // Add new method ptr
            methods_ptrs.push(ejr::JSMethod{
                cb: Some(global_static_callback_wrappper),
                name: name_cstrs.last().unwrap().as_ptr(),
                opaque: oo_ptr
            });
        }

        // Box the methods
        let mut methods_box = methods_ptrs.into_boxed_slice();
        let ptr = methods_box.as_mut_ptr();

        unsafe {
            ejr::ejr_register_module(self.rt, module_name_cstr.as_ptr(), ptr, methods.len());
        }
    }

    pub fn free_jsvalue(&self, value_id: i32) {
        unsafe {
            ejr::ejr_free_jsvalue(self.rt, value_id);
        }
    }

    /// Get a JSArg from a JSValue
    pub fn jsarg_from_jsvalue(&self, value: i32) -> *mut ejr::JSArg {
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

        // Free oo_ptrs
        for &ptr in self.opaque_objects.iter() {
            if !ptr.is_null() {
                unsafe {
                    let oo_ptr = ptr as *mut OpaqueObject;
                    let _ = Box::from_raw(oo_ptr); 
                }
            }
        }
        self.opaque_objects.clear();
    }
}