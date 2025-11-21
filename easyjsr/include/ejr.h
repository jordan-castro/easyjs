#ifndef EJR_C_API_H
#define EJR_C_API_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// Types

/**
 * @brief The C wrapper for EasyJSR
 */
typedef struct EasyJSRHandle EasyJSRHandle;

/**
 * @brief C version of our JSArg union
 */
typedef enum {
    JSARG_TYPE_INT,
    JSARG_TYPE_DOUBLE,
    JSARG_TYPE_STRING,
    JSARG_TYPE_FLOAT,
    JSARG_TYPE_BOOL,
    JSARG_TYPE_INT64_T,
    JSARG_TYPE_UINT32_T,
    JSARG_TYPE_C_ARRAY,
    JSARG_TYPE_NULL,
    JSARG_TYPE_UNDEFINED,
    JSARG_TYPE_UINT8_ARRAY,
    JSARG_TYPE_INT32_ARRAY,
    JSARG_TYPE_UINT32_ARRAY,
    JSARG_TYPE_INT64_ARRAY,
    JSARG_TYPE_INT8_ARRAY,
    JSARG_TYPE_UINT16_ARRAY,
    JSARG_TYPE_INT16_ARRAY,
    JSARG_TYPE_UINT64_ARRAY,
    JSARG_TYPE_FLOAT_ARRAY,
    JSARG_TYPE_EXCEPTION
} JSArgType;
typedef struct JSArg JSArg;
/**
 * @brief C version of our JSArg union
 */
struct JSArg {
    JSArgType type;
    union {
        int int_val;
        double double_val;
        const char* str_val;
        int jsvalue_id;
        float float_val;
        bool bool_val;
        uint32_t uint32_t_val;
        int64_t int64_t_val;
        struct {
            JSArg** items;
            size_t count;
            size_t capacity;
        } c_array_val;
        struct {
            const uint8_t* items;
            size_t count;
        } u8_array_val;
        struct {
            const int32_t* items;
            size_t count;
        } i32_array_val;
        struct {
            const uint32_t* items;
            size_t count;
        } u32_array_val;
        struct {
            const int64_t* items;
            size_t count;
        } i64_array_val;
        struct {
            const int8_t* items;
            size_t count;
        } i8_array_val;
        struct {
            const int16_t* items;
            size_t count;
        } i16_array_val;
        struct {
            const uint16_t* items;
            size_t count;
        } u16_array_val;
        struct {
            const uint64_t* items;
            size_t count;
        } u64_array_val;
        struct {
            const float* items;
            size_t count;
        } float_array_val; 
        struct {
            const char* msg;
            const char* name;
        } exception_val;
    } value;
};
/**
 * @brief C Callback wrapper for DynCallback 
 */
typedef JSArg* (*C_Callback)(JSArg** args, size_t arg_count, void* opaque);

/**
 * @brief C wrapper for FileLoaderFn
 */
typedef char* (*C_FileLoaderFn)(const char* file_path, void* opaque);

/**
 * @brief C version of JSMethod
 */
typedef struct JSMethod JSMethod;

/**
 * @brief C version of JSMethod
 */
struct JSMethod {
    const char* name;
    C_Callback cb;
    void* opaque;
};

// Constructors

/**
 * @brief Create a new easyjs runtime.
 * 
 * It will have it's own callables, modules, classes, etc. It is all sandboxed within itself.
 */
EasyJSRHandle* ejr_new();

/**
 * @brief Create a int JSArg.
 * 
 * @param value The int
 * 
 * @return JSArg
 */
JSArg* jsarg_int(int value);

/**
 * @brief Create a const char* JSArg.
 * 
 * This copies the value so feel free to delete afterwards.
 * 
 * @param value The const char*
 * 
 * @return JSArg
 */
JSArg* jsarg_str(const char* value);

/**
 * @brief Create a double JSArg.
 * 
 * @param value The double
 * 
 * @return JSArg
 */
JSArg* jsarg_double(double value);

/**
 * @brief Create a float JSArg.
 * 
 * @param value The float
 * 
 * @return JSArg
 */
JSArg* jsarg_float(float value);

/**
 * @brief Create a int64_t JSArg.
 * 
 * @param value The int64_t
 * 
 * @return JSArg
 */
JSArg* jsarg_int64t(int64_t value);

/**
 * @brief Create a uint32_t JSArg.
 * 
 * @param value The uint32_t
 * 
 * @return JSArg
 */
JSArg* jsarg_uint32t(uint32_t value);

/**
 * @brief Create a C-Array JSArg.
 * 
 * @param count The size of the array
 * 
 * @return JSArg
 */
JSArg* jsarg_carray(size_t count);

/**
 * @brief Create a null JSArg.
 * 
 * @return JSArg
 */
JSArg* jsarg_null();

/**
 * @brief Create a bool JSArg.
 * 
 * @param value The bool
 * 
 * @return JSArg
 */
JSArg* jsarg_bool(bool value);

/**
 * @brief Create a JSArgTypedArray<uint8_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The uint8_t*
 * @param argc The number of uint8_ts.
 * 
 * @return JSArg
 */
JSArg* jsarg_u8_array(const uint8_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<int32_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The int32_t*
 * @param argc The number of int32_t.
 * 
 * @return JSArg
 */
JSArg* jsarg_i32_array(const int32_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<uint32_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The uint32_t*
 * @param argc The number of uint32_t.
 * 
 * @return JSArg
 */
JSArg* jsarg_u32_array(const uint32_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<int64_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The int64_t*
 * @param argc The number of int64_t.
 * 
 * @return JSArg
 */
JSArg* jsarg_i64_array(const int64_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<int8_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The int8_t*
 * @param argc The number of int8_t.
 * 
 * @return JSArg
 */
JSArg* jsarg_i8_array(const int8_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<int16_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The int16_t*
 * @param argc The number of int16_t.
 * 
 * @return JSArg
 */
JSArg* jsarg_i16_array(const int16_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<uint16_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The uint16_t*
 * @param argc The number of uint16_t.
 * 
 * @return JSArg
 */
JSArg* jsarg_u16_array(const uint16_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<uint64_t>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The uint64_t*
 * @param argc The number of uint64_t.
 * 
 * @return JSArg
 */
JSArg* jsarg_u64_array(const uint64_t* args, size_t argc);

/**
 * @brief Create a JSArgTypedArray<float>. 
 * 
 * This will copy the memory so feel very free to free it.
 * 
 * @param args The float*
 * @param argc The number of float.
 * 
 * @return JSArg
 */
JSArg* jsarg_float_array(const float* args, size_t argc);

/**
 * @brief Create a JSArgException.
 * 
 * This will copy the memory so feel free to delete after this.
 * 
 * @param message The exceptions message
 * @param name The exceptions name
 * 
 * @return JSArg
 */
JSArg* jsarg_exception(const char* message, const char* name);

/**
 * @brief Add a JSArg value to a array.
 * 
 * @param arg Pointer to the array.
 * @param value the JSArg value.
 */
void jsarg_add_value_to_c_array(JSArg* arg, JSArg* value);

/**
 * @brief Get a JSArg from a JSValue(int)
 * 
 * @param handle the Easyjs runtime
 * @param value the JSValue(int)
 * 
 * @return JSArg
 */
JSArg* jsarg_from_jsvalue(EasyJSRHandle* handle, int value);

/**
 * @brief Create a undefined JSArg.
 *
 * @return JSArg 
 */
JSArg* jsarg_undefined();

// Deleters
/**
 * @brief Free a easyjs runtime
 */
void ejr_free(EasyJSRHandle* handle);

/**
 * @brief Free a JSArg.
 * 
 * @param arg A pointer to the JSArg to free.
 */
void jsarg_free(JSArg* arg);

/**
 * @brief Free A JSArg**
 * 
 * @param args A pointer of pointers of JSArgs
 * @param argc Number of pointers.
 */
void jsarg_free_all(JSArg** args, size_t argc);

/**
 * @brief Create a JSArg**
 * 
 * @param argc Count of args this can hold
 * 
 * @return a Pointer of Pointers
 */
JSArg** jsarg_make_list(size_t argc);

/**
 * @brief Add a JSArg to a JSArg** 
 * 
 * This is NOT for a C_Array! But rather a list of JSArgs
 * 
 * @param jsarg the list ptr
 * @param njsarg the jsarg to add
 * @param i the index where this goes
 */
void jsarg_add_to_list(JSArg** jsarg, JSArg* njsarg, size_t i);

// EasyJSR specific
/**
 * @brief Set the file_loader function.
 * 
 * @param handle the easyjsr runtime.
 * @param fn The file loader function.
 * @param opaque some opaque data.
 * 
 */
void ejr_set_file_loader(EasyJSRHandle* handle, C_FileLoaderFn fn, void* opaque);

/**
 * @brief Evaluate a JS script at the global level.
 * 
 * @param handle the easyjsr runtime.
 * @param js The JS code.
 * @param file_name The name of the file.
 * 
 * @return The id of the created JSValue.
 */
int ejr_eval_script(EasyJSRHandle* handle, const char* js, const char* file_name);

/**
 * @brief Evaluate a JS script as a module level.
 * 
 * @param handle the easyjsr runtime.
 * @param js The JS code.
 * @param file_name The name of the file.
 * 
 * @return The id of the created JSValue.
 */
int ejr_eval_module(EasyJSRHandle* handle, const char* js, const char* file_name);

/**
 * @brief Evaluate a JS function in the current scope/runtime.
 * 
 * IMPORTANT: args are freed during this call. If you plan to use them again, copy them.
 * 
 * @param handle the easyjsr runtime.
 * @param fn_name The JS function name.
 * @param args The args to pass into the function.
 * @param arg_count Number of args being passed.
 * 
 * @return The id of the resulted JSValue.
 */
int ejr_eval_function(EasyJSRHandle* handle, const char* fn_name, JSArg** args, size_t arg_count);

/**
 * @brief Convert a JSValue into a c_string.
 * 
 * @param handle the easyjsr runtime.
 * @param value_id The id of the JSValue to be converted.
 * 
 * @return The c_string.
 */
char * ejr_val_to_string(EasyJSRHandle* handle, int value_id);

/**
 * @brief Evaluate a JS function in the current scope/runtime on a class or object.
 * 
 * IMPORTANT: args are freed during this call. If you plan to use them again, copy them.
 * 
 * @param handle the easyjsr runtime.
 * @param value_id The objects/classes id in jsvad
 * @param fn_name The JS function name.
 * @param args The args to pass into the function.
 * @param arg_count Number of args being passed.
 * 
 * @return The id of the resulted JSValue.
 */
int ejr_eval_class_function(EasyJSRHandle* handle, int value_id, const char* fn_name, JSArg** args, size_t arg_count);

/**
 * @brief Get a property from a object.
 * 
 * @param handle the easyjsr runtime.
 * @param value_id The objects/classes id in jsvad
 * @param property The property name.
 * 
 * @return the Id of the resulted value.
 */
int ejr_get_property_from(EasyJSRHandle* handle, int value_id, const char* property);

/**
 * @brief Get a property from Global scope.
 * 
 * @param handle the easyjsr runtime.
 * @param property The property name.
 * 
 * @return the Id of the resulted value.
 */
int ejr_get_from_global(EasyJSRHandle* handle, const char* property);

/**
 * @brief Register a callback in JS.
 * 
 * @param handle The easyjsr runtime.
 * @param fn_name Name to give the callback.
 * @param cb The actual C callback.
 * @param opaque Opaque user data.
 */
void ejr_register_callback(EasyJSRHandle* handle, const char* fn_name, C_Callback cb, void* opaque);

/**
 * @brief Register a module in JS.
 * 
 * @param handle The easyjsr runtime.
 * @param module_name Name to give the module.
 * @param methods A array of methods to add to the runtime.
 * @param method_count number of methods.
 */
void ejr_register_module(EasyJSRHandle* handle, const char* module_name, JSMethod* methods, size_t method_count);

/**
 * @brief Await a JSValue. Does not free the passed in value.
 * 
 * @param handle The easyjsr runtime.
 * @param value_id The Promise value
 * 
 * @return the Id of the awaited promise.
 */
int ejr_await_promise(EasyJSRHandle* handle, int value_id);

/**
 * @brief Free a C string.
 * 
 * @param c_string The c string to free.
 */
void ejr_free_string(char* c_string);

/**
 * @brief Free a JSValue within the JSValueAD.
 * 
 * @param handle The easyjsr runtime associated.
 * @param value_id id of the value to free.
 */
void ejr_free_jsvalue(EasyJSRHandle* handle, int value_id);

#ifdef __cplusplus
}
#endif /// __cplusplus

#endif // EJR_C_API_H