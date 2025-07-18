// EasyJS STD version 0.4.2
const DATE: &str = r##"// Get the days between 2 dates
macro days_between_dates(d1, d2) { 
    Math.ceil(Math.abs(#d1 - #d2) / (1000 * 60 * 60 * 24)) 
}

// Get the weekday of a date.
macro get_week_day(d) { 
    #d.toLocaleString('en-US', {weekday: 'long'}) 
}

// Is a date a weekend?
macro is_weekend(d) {
    [5,6].indexOf(#d.getDay()) != -1
}"##;
const MALLOC: &str = r##"native {
    pub fn malloc(size:int):int {
        var ptr : int = 0
        __set_local_to_global(0, 0)

        // TODO: allow this!
        HEAP += ptr + 4 + size

        __get_global(0)
        __get_local(0)
        __i32_const(4)
        __i32_add()
        __set_global(0)

        return ptr
    }

}"##;
const MATH: &str = r##"macro radians(degrees) {
    javascript{
        #degrees * (Math.PI / 180);
    }
}

// Calculate the percentage in EasyJS.
macro calculate_percent(value,total) {
    Math.round((#value / #total) * 100)
}
"##;
const NATIVE: &str = r##"import 'std'

// Macro for encoding UTF-8
macro utf8_encode(str) {
    new TextEncoder('utf-8').encode(#str)
}

// Macro for decoding UTF-8
macro utf8_decode(bytes) {
    new TextDecoder('utf-8').decode(#bytes)
}

/// Initialize the native module.
async fn EASYJS_NATIVE_init(binary) {
    @const(module, await WebAssembly.instantiate(binary.buffer))
    @const(instance, module.instance)

    return instance.exports
}

/// Convert a host string to a native string
fn EASYJS_NATIVE_convert_string_to_native(instance, str) {
    // get length and bytes
    @const(str_len, str.length)
    @const(str_bytes, @utf8_encode(str))

    // allocate space and get ptr
    @const(ptr, instance.exports.__str_alloc(str_len))

    // store length
    instance.exports.__str_store_len(ptr, str_len)

    // Write the string to memory
    for i in 0..str_bytes.length {
        instance.exports.__str_store_byte(ptr, 4 + i, str_bytes[i])
    }

    return ptr
}

/// Convert a native string into a host string.
fn EASYJS_NATIVE_convert_string_to_host(instance, ptr) {
    @const(length, instance.exports.__str_get_len(ptr))
    @const(memory_buffer, new Uint8Array(instance.exports.memory.buffer, ptr + 4, length))

    // Decode string
    return @utf8_decode(memory_buffer)
}

/// Call a easyjs native method
fn EASYJS_NATIVE_call(instance, fn_name, param_types, return_types, ...args) {
    if !instance {
        @throw('No native module loaded')
    }

    if !instance.exports[fn_name] {
        @throw('Function $fn_name not found in native module')
    }

    if param_types.length != args.length {
        @throw('Number of arguments does not match number of parameters')
    }

    // Go through params and make sure args match type
    for i in 0..args.length {
        arg = args[i]
        param_type = param_types[i]

        match param_type {
            'string': {
                if typeof(arg) != 'string' {
                    @throw('Argument $i is not a string')
                }
                // This is a string so we need to convert it to a native pointer.
                args[i] = EASYJS_NATIVE_convert_string_to_native(instance, args[i])
            }
            'int': {
                if typeof(arg) != 'number' or !Number.isInteger(arg) {
                    @throw('Argument $i is not a integer')
                }
            }
            'float': {
                if typeof(arg) != 'number' or isNaN(arg) {
                    @throw('Argument is not a valid float')
                }
            }
            'bool': {
                // booleans must be true/false or a number
                if not @is_type(arg, 'boolean') and not @is_type(arg, 'number') {
                    @throw('Argument: $i is not a valid boolean')
                }
                // if true/false convert it to a int.
                if @is_type(arg, 'boolean') {
                    // TODO: implement this: args[i] = if arg == true 1 else 0
                    if arg == true {
                        args[i] = 1
                    } else {
                        args[i] = 0
                    }
                } else {
                    // is already a number so make sure it is either 0 or 1
                    if arg > 0 {
                        args[i] = 1
                    } else {
                        args[i] = 0
                    }
                }
            }
        }
    }
    // load result
    result = instance.exports[fn_name](...args)
    
    // match result type
    match return_types[0] {
        'string': {
            // convert from native to host
            result = EASYJS_NATIVE_convert_string_to_host(instance, result)
        }
        'bool': {
            if result == 1 {
                result = true
            } else {
                result = false
            }
        }
    }

    return result
}"##;
const RANDOM: &str = r##"// EasyJS implementation of random.uniform from Python.
macro uniform(a,b) {
    Math.random() * (#b - #a + 1) + #a
}

macro choice(array) {
    #array[Math.floor(Math.random() * #array.length)]
}

macro normal(mean, std_dev) {
    u1 = Math.random()
    u2 = Math.random()
    z0 = Math.sqrt(-2.0 * Math.log(u1)) * Math.cos(2.0 * Math.PI * u2) // Box-Muller transform
    z0 * #std_dev + #mean
}

// Shuffle an array randomly.
macro shuffle(arr) {
    #arr.slice().sort(fn() {
        return Math.random() - 0.5
    })
}

// Get a random number from min max
macro random_number(min, max) {Math.floor(Math.random() * (#max - #min + 1) + #min)}

// Get a random hex color
macro random_hex_color() { "#${Math.random().toString(16).slice(2, 8).padEnd(6, '0')}"}

// Get a Random boolean
macro random_bool() { Math.random() >= 0.5}

"##;
const STD: &str = r##"// Get the last element of an array
macro last(array) {
    #array[#array.length - 1]
}

macro print(msg) {
    console.log(#msg)
}

// Get the first element of an array
macro first(array) {
    #array[0]
}

macro throw(error_msg) {
    javascript {
        throw new Error(#error_msg);
    }
}

// Try to do an operation.
macro try(method, throw) {
    javascript {
        try {
            #method();
        } catch (e) {
            if (#throw == true) {
                throw e;
            }
        }
    }
}

// Decouple 2 objects. 1 of identifiers, and 1 of matching length/key of values.
macro decouple(idents, values) {
    #idents = #values
}

// declare a constant variable 
macro const(ident, value) {
    javascript {
        const #ident = #value;
    }
}

macro run_function(fun) {
    #fun()
}

macro sleep(ms) {
    @const(func, fn(ms) {
        javascript{
            return new Promise(resolve => setTimeout(resolve, ms))
        }
    })

    await func(#ms)
}

// Creates a range
macro range(kwargs) {
    @run_function(fn() {
        start = #kwargs.start
        end = #kwargs.end
        step = #kwargs.step ?? 1

        javascript{
            return Array(Math.ceil((end - start) / step)).fill(start).map((x,y) => x + y * step)
        }
    })
}

// Flatten a list in JS.
macro flatten(list) {
    javascript {
        [...new Set(#list)];
    }
}

// Capitalize a string
macro capitalize(str) {
    "${#str.charAt(0).toUpperCase()}${#str.slice(1)}"
}

// Merge 2 arrays
macro merge(arr1, arr2, flatten) {
    narr = [].concat(#arr1, #arr2)

    fn {
        if #flatten == true {
            return flat(narr)
        }

        return narr
    }
}

// Reverse a string
macro reverse_string(str) {
    fn {
        return #str.split("").reverse().join("")
    }
}

// Get the EasyJS ASCII
macro easyjs_ascii() {
    "    ___       ___       ___       ___            ___       ___   
   /\\  \\     /\\  \\     /\\  \\     /\\__\\          /\\  \\     /\\  \\  
  /::\\  \\   /::\\  \\   /::\\  \\   |::L__L        _\\:\\  \\   /::\\  \\ 
 /::\\:\\__\\ /::\\:\\__\\ /\\:\\:\\__\\  |:::\\__\\      /\\/::\\__\\ /\\:\\:\\__\\
 \\:\\:\\/  / \\/\\::/  / \\:\\:\\/__/  /:;;/__/      \\::/\\/__/ \\:\\:\\/__/
  \\:\\/  /    /:/  /   \\::/  /   \\/__/          \\/__/     \\::/  / 
   \\/__/     \\/__/     \\/__/                              \\/__/  "
}

// add a to_string method for a enum
macro add_to_string_to_enum(enum_name) {
    /// Convert #enum_name to a String representation.
    fn #enum_name_to_string(val) {
        keys = Object.keys(#enum_name)
        for key in keys {
            if #enum_name[key] == val {
                return key
            }
        }

        // Was not found
        return null
    }
}

// Call Object.keys on a object
macro keys(object) {
    Object.keys(#object)
}

// Call Object.freeze on a object
macro freeze(object) {
    Object.freeze(#object)
}

// Check type
macro is_type(variable, type_name) {
    typeof(#variable) == #type_name
}"##;
const STRINGS: &str = r##"native {
    pub fn __get_str_len(ptr:int):int {
        EJ_local_get(EJ_local_from_ident(ptr))
        __i32_load_8u(0,0,0)
        // automatically returned at the end
    }

    // concat 2 strings together, returns a new pointer!
    pub fn __concat(ptr1:int, ptr2:int):int {
        var string1_len = __get_str_len(ptr1)
        var string2_len = __get_str_len(ptr2)
    }
}"##;

/// Load a STD library from EasyJS version 0.4.2, or an empty string if not found.
pub fn load_std(name: &str) -> String {
match name {
"date" => DATE,
"malloc" => MALLOC,
"math" => MATH,
"native" => NATIVE,
"random" => RANDOM,
"std" => STD,
"strings" => STRINGS,
_ => "",
}.to_string()}