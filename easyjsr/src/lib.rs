#![allow(warnings)]

mod ejr {
    include!("../bindings.rs");
}
use std::{any::Any, collections::{btree_map::Values, HashMap}, ffi::{CStr, CString}, os::raw::{c_char, c_void}, ptr::{slice_from_raw_parts, slice_from_raw_parts_mut, NonNull}, sync::{Arc, Mutex}};

lazy_static::lazy_static! {
    static ref RUNTIME_REGISTRY: Mutex<RTGlobalContext> = Mutex::new(RTGlobalContext::new());
}

// TYPES
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSArgType {
    Int = 0,
    Double = 1,
    String = 2,
    Float = 3,
    Bool = 4,
    Int64 = 5,
    Uint32 = 6,
    CArray = 7,
    Null = 8,
    Undefined = 9,
    UInt8Array = 10,
    Int32Array = 11,
    UInt32Array = 12,
    Int64Array = 13,
    Int8Array = 14,
    UInt16Array = 15,
    Int16Array = 16,
    UInt64Array = 17,
    FloatArray = 18
}

impl JSArgType {
    pub fn c_val(&self) -> ejr::JSArgType {
        match self {
            JSArgType::Int => ejr::JSArgType_JSARG_TYPE_INT,
            JSArgType::Double => ejr::JSArgType_JSARG_TYPE_DOUBLE,
            JSArgType::String => ejr::JSArgType_JSARG_TYPE_STRING,
            JSArgType::Float => ejr::JSArgType_JSARG_TYPE_FLOAT,
            JSArgType::Bool => ejr::JSArgType_JSARG_TYPE_BOOL,
            JSArgType::Int64 => ejr::JSArgType_JSARG_TYPE_INT64_T,
            JSArgType::Uint32 => ejr::JSArgType_JSARG_TYPE_UINT32_T,
            JSArgType::CArray => ejr::JSArgType_JSARG_TYPE_C_ARRAY,
            JSArgType::Null => ejr::JSArgType_JSARG_TYPE_NULL,
            JSArgType::Undefined => ejr::JSArgType_JSARG_TYPE_UNDEFINED,
            JSArgType::UInt8Array => ejr::JSArgType_JSARG_TYPE_UINT8_ARRAY,
            JSArgType::Int32Array => ejr::JSArgType_JSARG_TYPE_INT32_ARRAY,
            JSArgType::UInt32Array => ejr::JSArgType_JSARG_TYPE_UINT32_ARRAY,
            JSArgType::Int64Array => ejr::JSArgType_JSARG_TYPE_INT64_ARRAY,
            JSArgType::Int8Array => ejr::JSArgType_JSARG_TYPE_INT8_ARRAY,
            JSArgType::UInt16Array => ejr::JSArgType_JSARG_TYPE_UINT16_ARRAY,
            JSArgType::Int16Array => ejr::JSArgType_JSARG_TYPE_INT16_ARRAY,
            JSArgType::UInt64Array => ejr::JSArgType_JSARG_TYPE_UINT64_ARRAY,
            JSArgType::FloatArray => ejr::JSArgType_JSARG_TYPE_FLOAT_ARRAY,
        }
    }
}

/// Type for Loader function
type LoaderFn = Box<fn (String) -> String>;
pub type JSArg = *mut ejr::JSArg;
pub type Opaque = *mut c_void;
pub type OpaqueUserData = Arc<dyn Any + Send + Sync>;
pub type JSArgResult = Option<JSArg>;
/// Type for Rust callbacks
type RustCallbackFn = Box<dyn Fn(Vec<JSArg>, &OpaqueObject) -> Option<*mut ejr::JSArg> + Send + Sync>;

/// EJR wrapper
pub struct EJR {
    /// the actual runtime
    rt: *mut ejr::EasyJSRHandle,
    /// Position in the global runtime.
    ptr: u32,
    /// Callback ptrs
    ptrs: Vec<*mut CallbackData>,
}

/// Data for callbacks
struct CallbackData {
    /// Runtime id
    rtctx_id: u32,
    /// OpaqueObject id
    opaque_object_id: u32
}

/// Opaque Object
pub struct OpaqueObject {
    /// Callback
    cb: RustCallbackFn,
    /// Custom user defined data
    pub user_data: Option<OpaqueUserData>
}

/// JSMethod wrapper
pub struct JSMethod {
    /// method Name
    name: String,
    /// Actual callback
    method: RustCallbackFn,
}

/// Runtime context
struct RuntimeContext {
    opaque_map: HashMap<u32, OpaqueObject>,
    next_opaque_id: u32,

    file_loader: LoaderFn
}

/// Global context
struct RTGlobalContext {
    runtimes_map: HashMap<u32, RuntimeContext>,
    next_runtime_id: u32
}

impl RuntimeContext {
    fn new() -> Self {
        Self {
            file_loader: Box::new(|_| -> String {"".to_string()}),
            next_opaque_id: 0,
            opaque_map: HashMap::new()
        }
    }

    fn add_opaque(&mut self, opaque: OpaqueObject) -> u32 {
        let id = self.next_opaque_id;
        self.opaque_map.insert(id, opaque);
        self.next_opaque_id += 1;
        id
    }

    fn get_opaque(&self, id: u32) -> Option<&OpaqueObject> {
        if id >= self.next_opaque_id {
            None
        } else {
            self.opaque_map.get(&id)
        }
    }

    fn set_file_loader(&mut self, func: LoaderFn) {
        self.file_loader = func;
    }

    fn free(&mut self) {
        self.opaque_map.clear();
        self.next_opaque_id = 0;
    }
}

impl RTGlobalContext {
    fn new() -> Self {
        Self {
            next_runtime_id: 0,
            runtimes_map: HashMap::new()
        }
    }

    fn add_runtime(&mut self, rt: RuntimeContext) -> u32 {
        let id = self.next_runtime_id;
        self.runtimes_map.insert(id, rt);
        self.next_runtime_id += 1;
        id
    }

    fn get_runtime(&mut self, id: u32) -> &mut RuntimeContext {
        self.runtimes_map.get_mut(&id)
            .expect("Runtime ID not found")
    }

    fn delete_runtime(&mut self, id: u32) {
        self.runtimes_map.remove(&id);
    }

    fn free(&mut self) {
        for (_, rt) in self.runtimes_map.iter_mut() {
            rt.free();
        }
        self.runtimes_map.clear();
        self.next_runtime_id = 0;
    }
}

/// Convert a Rust &str into a CString
pub fn str_to_cstr(val: &str) -> CString {
    CString::new(val).expect("Could not convert val into cString")
}

pub fn cstr_to_string(val: *mut i8) -> String {
    unsafe {
        CStr::from_ptr(val).to_string_lossy().into_owned()
    }
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

macro_rules! jsarg_array_ptr {
    ($values:expr) => {{
        let boxed_slice = $values.into_boxed_slice();
        let ptr = boxed_slice.as_ptr();
        let len = boxed_slice.len();

        // Forget about it!
        std::mem::forget(boxed_slice);
        
        (ptr, len)
    }};
}

/// Create a JSArg value of u8 Array
pub fn jsarg_u8_array(values: Vec<u8>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_u8_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_u16_array(values: Vec<u16>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_u16_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_u32_array(values: Vec<u32>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_u32_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_u64_array(values: Vec<u64>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_u64_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_i16_array(values: Vec<i16>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_i16_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_i32_array(values: Vec<i32>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_i32_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_i64_array(values: Vec<i64>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_i64_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_i8_array(values: Vec<i8>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_i8_array(ptr, len)
    }
}

/// Create a JSArg value of u8 Array
pub fn jsarg_float_array(values: Vec<f32>) -> JSArg {
    let (ptr, len) = jsarg_array_ptr!(values);

    unsafe {
        ejr::jsarg_float_array(ptr, len)
    }
}

/// Create a JSArg value of Null
pub fn jsarg_null() -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_null()
    }
}

/// Create a JSArg value of Undefined
pub fn jsarg_undefined() -> *mut ejr::JSArg {
    unsafe {
        ejr::jsarg_undefined()
    }
}

/// Create a JSArg**
pub fn jsarg_list(items: Vec<*mut ejr::JSArg>) -> *mut *mut ejr::JSArg {
    unsafe {
       let jsarg_list = ejr::jsarg_make_list(items.len());
        for i in 0..items.len() {
            ejr::jsarg_add_to_list(jsarg_list, items[i], i);
        }

        jsarg_list
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
    let rtctx_id = unsafe { *(opaque as *const u32) };
    let mut reg = RUNTIME_REGISTRY.lock().unwrap();
    let mut rtctx = reg.get_runtime(rtctx_id);

    // Rust Conversion
    let file_path_rs = unsafe { CStr::from_ptr(file_path).to_string_lossy().into_owned()};

    // Run
    let result = (rtctx.file_loader)(file_path_rs);

    // Convert result to *mut c_char
    CString::new(result).unwrap().into_raw()
}

/// Global(static) function for calling a callback
unsafe extern "C" fn global_static_callback_wrappper(args: *mut *mut ejr::JSArg, argc: usize, opaque: *mut c_void) -> *mut ejr::JSArg {
    // Get RTCX
    let cb_data: &CallbackData = &*(opaque as *mut CallbackData);

    let mut reg = RUNTIME_REGISTRY.lock().unwrap();
    let mut rtctx = reg.get_runtime(cb_data.rtctx_id);

    // Get OO
    let oo = rtctx.get_opaque(cb_data.opaque_object_id).unwrap();
    
    // Call fn
    let cb_fn = &oo.cb;

    // RS Conversion
    let args_rs = unsafe {std::slice::from_raw_parts(args, argc) };

    let result = (cb_fn)(args_rs.to_vec(), oo);

    if result.is_none() {
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

        let rt_ctx = RuntimeContext::new();
        let mut reg = RUNTIME_REGISTRY.lock().unwrap();
        let mut rt_id = reg.add_runtime(rt_ctx);
        let mut ejr = Self {
            ptr: rt_id,
            rt: instance,
            ptrs: vec![]
        };
        let ptr = &mut rt_id as *mut u32;

        unsafe {
            ejr::ejr_set_file_loader(ejr.rt, Some(global_static_file_loader), ejr.get_ptr());
        }
        ejr
    }

    fn get_ptr(&mut self) -> *mut c_void {
        self.ptr as usize as *mut c_void
    }

    // Rust first methods
    
    // /// Get from user_data
    // pub fn get_user_data<T: 'static>(&self, id: u32) -> Option<&T> {
    //     self.user_data.get(&id)?.downcast_ref::<T>()
    // }

    // /// Get mut from user_data
    // pub fn get_user_data_mut<T: 'static>(&mut self, id: u32) -> Option<&mut T> {
    //     self.user_data.get_mut(&id)?.downcast_mut::<T>()
    // }

    // fn insert_user_data<T: 'static>(&mut self, data: T) -> u32 {
    //     let id = self.next_user_data_id;
    //     self.user_data.insert(id, Box::new(data));
    //     self.next_user_data_id += 1;

    //     id
    // }

    // fn insert_cb(&mut self, cb: RustCallbackFn) -> u32 {
    //     let id = self.next_cb_fn_id;
    //     self.cb_fns.insert(id, cb);
    //     self.next_cb_fn_id += 1;
        
    //     id
    // }

    // fn add_oo(&mut self, oo: *mut c_void) {
    //     self.opaque_objects.push(oo);
    // }

    /// Setup the file loader.
    pub fn set_file_loader(&self, func: LoaderFn) {
        let mut reg = RUNTIME_REGISTRY.lock().unwrap();
        let mut rt_ctx = reg.get_runtime(self.ptr);
        rt_ctx.file_loader = func;
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
        let args_ptr = jsarg_list(args);

        unsafe {
            let result = ejr::ejr_eval_function(self.rt, func_name_cstr.as_ptr(), args_ptr, args_len);
            result
        }
    }

    /// Convert a JSValue into a String.
    pub fn val_to_string(&self, value_id: i32) -> Option<String> {
        unsafe {
            let result = ejr::ejr_val_to_string(self.rt, value_id);
            if result.is_null() {
                None
            } else {
                Some(CString::from_raw(result).to_string_lossy().to_string())
            }
        }
    }

    /// Evalute a class/object/prototype function
    pub fn evall_class_function(&self, value_id: i32, func_name: &str, args: Vec<*mut ejr::JSArg>) -> i32 {
        // C Conversions
        let func_name_cstr = str_to_cstr(func_name);
        let args_len = args.len();
        let args_ptr = jsarg_list(args);

        unsafe {
            let result = ejr::ejr_eval_class_function(self.rt, value_id, func_name_cstr.as_ptr(), args_ptr, args_len);
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
    pub fn register_callback(&mut self, func_name: &str, cb: RustCallbackFn, opaque: Option<OpaqueUserData>) {
        // C conversion
        let func_name_cstr = str_to_cstr(func_name);

        // Get the Runtime context
        let mut binding = RUNTIME_REGISTRY.lock().unwrap();
        let mut rtctx = binding.get_runtime(self.ptr);
        // Save callback and opaque
        let oo = OpaqueObject{
            cb: cb,
            user_data: opaque
        };

        // Add oo to runtime
        let oo_id = rtctx.add_opaque(oo);

        let cb_data = CallbackData{
            opaque_object_id: oo_id,
            rtctx_id: self.ptr
        };

        // Convert cb_data into PTR
        let cb_box = Box::new(cb_data);
        let cb_ptr = Box::into_raw(cb_box);

        self.ptrs.push(cb_ptr);

        unsafe {
            ejr::ejr_register_callback(
                self.rt, 
                func_name_cstr.as_ptr(), 
                Some(global_static_callback_wrappper), 
                cb_ptr as *mut c_void
            );
        }
    }

    /// Register a JS Module with Rust callbacks
    pub fn register_module(&mut self, module_name: &str, methods: Vec<JSMethod>, user_data: Option<OpaqueUserData>) {
        // C conversions
        let module_name_cstr = str_to_cstr(module_name);
        let mut methods_ptrs = vec![];
        let mut name_cstrs = vec![];
        
        // Get runtime context
        let mut binding = RUNTIME_REGISTRY.lock().unwrap();
        let mut rtctx = binding.get_runtime(self.ptr);

        let method_length = methods.len();
            
        for m in methods {
            let name_cstr = str_to_cstr(m.name.as_str());
            // save in name_cstrs to not free yet
            name_cstrs.push(name_cstr);
            // Save callback
            let oo = OpaqueObject{
                cb: m.method,
                user_data: user_data.clone()
            };
            let ooid =rtctx.add_opaque(oo);

            let cb_data = CallbackData{
                opaque_object_id: ooid,
                rtctx_id: self.ptr
            };
            let cb_box = Box::new(cb_data);
            let cb_ptr = Box::into_raw(cb_box);
            self.ptrs.push(cb_ptr);

            // Add new method ptr
            methods_ptrs.push(ejr::JSMethod{
                cb: Some(global_static_callback_wrappper),
                name: name_cstrs.last().unwrap().as_ptr(),
                opaque: cb_ptr as *mut c_void
            });
        }

        // Box the methods
        let mut methods_box = methods_ptrs.into_boxed_slice();
        let ptr = methods_box.as_mut_ptr();

        unsafe {
            ejr::ejr_register_module(self.rt, module_name_cstr.as_ptr(), ptr, method_length);
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

    /// Free a C String
    pub fn free_string(&self, val: *mut i8) {
        unsafe {
            ejr::ejr_free_string(val);
        }
    }
}

impl Drop for EJR {
    fn drop(&mut self) {
        unsafe {
            ejr::ejr_free(self.rt);
        }

        // Delete ptrs
        for cb_ptr in self.ptrs.clone() {
            unsafe {
                let cb_box = Box::from_raw(cb_ptr);
            }
        }

        // Delete from global
        let mut binding = RUNTIME_REGISTRY.lock().unwrap();
        binding.delete_runtime(self.ptr);
    }
}