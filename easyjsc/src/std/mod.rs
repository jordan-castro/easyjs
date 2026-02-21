// EasyJS STD version 0.4.6
const HTML: &str = r##"// A full HTML DSL using macros
macro html(elements) {
    "<html>
        ${#elements}
    </html>"
}

macro head(elements) {
    "<head>${#elements}</head>"
}

macro title(title) {
    "<title>${#title}</title>"
}

macro body(elements, kwargs) {
    javascript{
        <body style='${#kwargs?.style ?? ""}'>
            #elements
        </body>
    }
}

macro h1(inner, kwargs) {
    "<h1 style='${#kwargs.style}'>${#inner}</h1>"
}

macro elements(els) {
    javascript{
        `${(#els.map(((e) => e)).join('\n'))}`
    }
}

"##;
const STD: &str = r##"// Get the last element of an array
macro last(array) {
    #array[#array.length - 1]
}

macro print(...msg) {
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
    ___try = #method
    javascript {
        try {
            ___try();
        } catch (e) {
            if (#throw == true) {
                throw e;
            }
        }
    }
}

// Try and catch a operation
macro try_catch(method, on_catch) {
    ___try = #method
    ___catch = #on_catch
    javascript {
        try {
            ___try();
        } catch (e) {
            ___catch(e)
        }
    }
}

// Decouple 2 objects. 1 of identifiers, and 1 of matching length/key of values.
macro decouple(idents, values) {
    #idents = #values
}

// declare a constant variable 
macro const(expr) {
    javascript {
        const ${#expr};
    }
}

macro run_function(fun) {
    #fun()
}

macro sleep(ms) {
    const!(func = fn(ms) {
        javascript{
            return new Promise(resolve => setTimeout(resolve, ms));
        }
    })

    await func(#ms)
}

// Flatten a list in JS.
macro flatten(list) {
    javascript {
        [...new Set(#list)];
    }
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

// Call Object.keys on a object
macro keys(object) {
    Object.keys(#object)
}

// Call Object.freeze on a object
macro freeze(object) {
    Object.freeze(#object)
}

// console.error macro
macro eprint(err) {
    console.error(#err)
}

// Keep comment alive after compilation by wrapping it in a javascript block
macro jsc(comment) {
    javascript{
        // #comment
    }
}

// JS eq `===`
macro jseq(object, value) {
    javascript{
        (#object === #value)
    }
}

// Is none (null or undefined)
macro is_none(object) {
    jseq!(#object, null) or jseq!(#object, undefined)
}

macro len(obj) {
    #obj.length
}"##;
const AGENTS: &str = r##"// Property of easyjs

class Agent {
    name:string
    tools:Array<Tool>

    fn __new(self, details) {
        self.name = details.name ?? ''
        self.tools = details.tools ?? []
    }
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
const NAT___STD: &str = r##"native {
    pub fn set_ptr(position:int, value:int) {
        __i32_store(value, position, 0)
    }

    pub fn get_ptr(position:int):int {
        __i32_load(position, 0)
    }
}"##;
const NAT___STRINGS: &str = r##""##;
const NAT___MEM: &str = r##"native {
    /// Global heap
    pub HEAP:int = 1024
    /// Global Page size 
    pub PAGE_SIZE:int = 65536
    /// Global nullptr
    pub nullptr:int = 0

    /// Int type
    pub IntType = 1
    /// Float type
    pub FloatType = 2
    /// String type
    pub StringType = 3
    /// Array type
    pub ArrayType = 4
    /// Dict type
    pub DictType = 5
    /// Struct type
    pub StructType = 6

    /// Set a PTR. I32 only
    pub fn set_ptr(position:int, value:int) {
        __i32_store__(position, value)
    }

    /// Get a PTR. I32 only
    pub fn get_ptr(position:int):int {
        return __i32_load__(position)
    }

    /// Allocate something on the heap.
    /// Simply grows the size of the heap by the requested amount.
    pub fn malloc(size:int):int {
        ptr = HEAP
        HEAP += size
        return ptr
    } 

    /// Set variable type
    pub fn set_type(ptr:int, var_type:int) {
        set_ptr(ptr, var_type)
    }

    /// Get variable type
    pub fn get_type(ptr:int):int {
        if ptr == nullptr {
            return nullptr
        }

        return get_ptr(ptr)
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
const STRINGS: &str = r##"// String manipulations

// Capitalize a string
macro make_capital(str) {
    "${#str.charAt(0).toUpperCase()}${#str.slice(1)}"
}"##;
const SYS: &str = r##"// copywright of easyjs

import 'std' as _

/// Command line args that do not include the file_name or runtime
@const(args = process.argv.slice(2, process.argv.length))
/// File name
@const(file_name = process.argv[1])

/// Execute a shell command.
macro exec(command) {
    (async fn() {
        err = null
        stdout = null
        stderr = null
        // use exec
        if ___runtime == 'deno' {
            split_command = #command.split(' ')
            @const(command = new Deno.command(split_command[0], {
                args: [
                    ...split_command[1..]
                ]
            }))
            @const(result = await command.output())
            err = result.code
            stdout = result.stdout
            stderr = result.stderr
        } else {
            // We use node otherwise
            @const({exec} = require('child_process'))

            exec(#command, fn(e, so, se) {
                err = e
                stdout = so
                stderr = se
            })
        }

        return { err, stdout, stderr }
    })()
}"##;

/// Load a STD library from EasyJS version 0.4.6, or an empty string if not found.
pub fn load_std(name: &str) -> String {
	match name {
		"html" => HTML,
		"std" => STD,
		"agents" => AGENTS,
		"random" => RANDOM,
		"nat/std" => NAT___STD,
		"nat/strings" => NAT___STRINGS,
		"nat/mem" => NAT___MEM,
		"math" => MATH,
		"date" => DATE,
		"strings" => STRINGS,
		"sys" => SYS,
		_ => "",
	}.to_string()
}