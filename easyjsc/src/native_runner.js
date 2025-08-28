const __easyjs_native = new WebAssembly.Module(__easyjs_native_module);
const __easyjs_native_instance = new WebAssembly.Instance(__easyjs_native);
const __easyjs_native_i32_type = 0;
const __easyjs_native_f32_type = 1;
const __easyjs_native_string_type = 2;
const __easyjs_native_array_type = 3;

class __EasyJSNativeInterop {
    /**
     * Function for converting a string to native.
     */
    static convert_string_to_native(instance, str) {
        // get length and bytes
        const strLen = str.length;
        const strBytes = new TextEncoder('utf-8').encode(str);

        // allocate space and get pointer
        const ptr = instance.exports.__str_alloc(strLen);

        // store length
        instance.exports.__str_store_len(ptr, strLen);

        // Write the string to memory
        for (let i = 0; i < strBytes.length; i++) {
            instance.exports.__str_store_byte(ptr, 4 + i, strBytes[i]);
        }
        return ptr;
    }

    /**
     * Function for converting a array to native.
     */
    static convert_array_to_native(instance, array) {
        // Get length
        const arrLen = array.length;

        // Allocate space and get pointer
        const ptr = instance.exports.__arr_alloc(arrLen * 2);

        // Store length
        instance.exports.__arr_store_len(ptr, arrLen);

        // Store cap
        instance.exports.__arr_store_cap(ptr, arrLen * 2);

        // Write items to memory
        for (let i = 0; i < arrLen; i++) {
            const arg = array[i];
            // Check type
            if (typeof(arg) == "number") {
                // Check integer?
                if (Number.isInteger(arg)) {
                    instance.exports.__arr_push_int(ptr, arg);
                } else {
                    // this is a float
                    instance.exports.__arr_push_float(ptr, arg);
                }
            } else if (typeof(arg) == "boolean") {
                // as integer
                instance.exports.__arr_push_int(ptr, arg);
            } else if (Array.isArray(arg)) {
                // run this again and pass in the ptr
                instance.exports.__arr_push_array(ptr, this.convert_array_to_native(instance, arg));
            } else if (typeof(arg) == "string") {
                // convert to native and pass in the ptr
                instance.exports.__arr_push_string(ptr, this.convert_string_to_native(instance, arg));
            }
        }

        return ptr;
    }

    /**
     * Function for reading a string from native.
     */
    static read_string_from_native(instance, ptr) {
        const length = instance.exports.__str_get_len(ptr);

        const memoryBuffer = new Uint8Array(instance.exports.memory.buffer, ptr + 4, length);

        // Decode the string
        const decodedString = new TextDecoder('utf-8').decode(memoryBuffer);

        return decodedString;
    }

    /**
     * Function for reading a array from native.
     */
    static read_array_from_native(instance, ptr) {
        const arrayLength = instance.exports.__arr_get_len(ptr);
        const arrayOffset = ptr + 8;
        const memoryI32 = new Int32Array(instance.exports.memory.buffer, arrayOffset, arrayLength * 2);
        const memoryF32 = new Float32Array(instance.exports.memory.buffer, arrayOffset, arrayLength * 2);
        
        // The decoded array
        let result = [];
    
        // Loop through items
        for (let i = 0; i < arrayLength; i++) {
            const base = i * 2;

            const argType = memoryI32[base];
            
            switch (argType) {
                case __easyjs_native_i32_type: {
                    result.push(memoryI32[base + 1]);
                    break;
                }
                case __easyjs_native_f32_type: {
                    result.push(memoryF32[base + 1]);
                    break;
                }
                case __easyjs_native_string_type: {
                    // decode native string
                    result.push(this.read_string_from_native(instance, memoryI32[base + 1]));
                    break;
                }
                case __easyjs_native_array_type: {
                    // decode native array
                    result.push(this.read_array_from_native(instance, memoryI32[base + 1]));
                    break;
                }
            }
        }

        return result;
    }
}

function __easyjs_native_call(fnName, paramTypes, returnTypes, ...args) {
    if (!__easyjs_native_instance) {
        throw new Error('No instance of __easyjs_native loaded');
    }

    if (!__easyjs_native_instance.exports[fnName]) {
        throw new Error(`Function ${fnName} not found in __easyjs_native`);
    }

    if (paramTypes.length !== args.length) {
        throw new Error('Number of arguments does not match number of parameters');
    }

    // go through params and make sure args match type
    for (let i = 0; i < args.length; i++) {
        const arg = args[i];
        const paramType = paramTypes[i];

        switch (paramType) {
            case 'string': {
                if (typeof arg !== 'string') {
                    throw new Error(`Argument ${i} is not a string`);
                }

                // this is a string so we need to convert it to a native pointer.
                args[i] = __EasyJSNativeInterop.convert_string_to_native(__easyjs_native_instance, args[i]);
                break;
            }
            case 'int': {
                if (typeof arg !== 'number' || !Number.isInteger(arg)) {
                    throw new Error(`Argument ${i} is not an integer`);
                }
                break;
            }
            case 'float': {
                if (typeof arg !== 'number' || isNaN(arg)) {
                    throw new Error(`Argument ${i} is not a valid float`);
                }
                break;
            }
            case 'bool': {
                // booleans must be true/false or a number
                if (typeof arg !== 'boolean' && typeof arg !== 'number') {
                    throw new Error(`Argument ${i} is not a valid boolean`);
                }
                // if true/false convert it to a int
                if (typeof arg === 'boolean') {
                    args[i] = arg == true ? 1 : 0;
                } else {
                    // make sure that the value is 0 or 1
                    args[i] = arg > 0 ? 1 : 0;
                }
                break;
            }
            case 'array': {
                if (!Array.isArray(arg)) {
                    throw new Error(`Argument ${i} is not a array`);
                }

                // converrt this into a native pointer
                args[i] = __EasyJSNativeInterop.convert_array_to_native(__easyjs_native_instance, args[i]);
                break;
            }
        }
    }

    let result = __easyjs_native_instance.exports[fnName](...args);

    // match result type
    // TODO: support multiple return types
    switch (returnTypes[0]) {
        case 'string': {
            // get length
            result = __EasyJSNativeInterop.read_string_from_native(__easyjs_native_instance, result);
            break;
        }
        case 'int': {
            break;
        }
        case 'float': {
            break;
        }
        case 'bool': {
            result = result == 0 ? false : true
            break;
        }
        case 'array': {
            result = __EasyJSNativeInterop.read_array_from_native(__easyjs_native_instance, result);
            break;
        }
    }

    return result;
}

