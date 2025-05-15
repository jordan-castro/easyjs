// EasyJS STD version 0.4.0
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

/// Load a STD library from EasyJS version 0.4.0, or an empty string if not found.
pub fn load_std(name: &str) -> String {
match name {
"date" => DATE,
"malloc" => MALLOC,
"math" => MATH,
"random" => RANDOM,
"std" => STD,
"strings" => STRINGS,
_ => "",
}.to_string()}