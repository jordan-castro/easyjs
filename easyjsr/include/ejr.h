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
        } c_array_val;
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