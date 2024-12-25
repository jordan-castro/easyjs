// EasyJS STD version 0.2.4
const UI: &str = r##"// Used for creating EasyJS webapps.
struct Children {
    fn constructor() {
        self.elements = []
        self.id_to_position = {}
    }

    fn add_one(el) {
        el_pos = @len(self.elements)
        if el.id ?? false {
            self.id_to_position[el.id] = el_pos
        }

        self.elements.push(el)
    }

    fn add_many(elements) {
        for el in elements {
            self.add_one(el)
        }
    }
}

struct HTMLElement {
    fn constructor(tag_name) {
        self.tag_name = tag_name
        self.children = Children()
    }

    fn add(element) {
        if @is(element, "array") {
            self.children.add_many(element)
        } else {
            self.children.add_one(element)
        }
    }
}"##;
const BUILTINS: &str = r##"pub fn int_range(start, end) {
    var res = []
    // the .. works because this is a for loop...
    for i in start..end {
        res.push(i)
    }

    return res
}

pub fn sleep(ms) {
    javascript{
        return new Promise(resolve => setTimeout(resolve, ms))
    }
}

/// Creates a range
pub fn range(kwargs) {
    start = kwargs.start
    end = kwargs.end
    step = kwargs.step ?? 1

    javascript{
        return Array(Math.ceil((end - start) / step)).fill(start).map((x,y) => x + y * step)
    }
}

/// Flatten a list in JS.
pub fn flat(list) {
    javascript{
        return [...new Set(list)];
    }
}

/// Capitalize a string
pub fn capitalize(str) {
    return "${str.charAt(0).toUpperCase()}${str.slice(1)}"
}

/// Merge 2 arrays
pub fn merge(arr1, arr2, flatten) {
    var narr = [].concat(arr1, arr2)

    if flatten == true {
        return flat(narr)
    }

    return narr
}

/// Reverse a string
pub reverse_string = fn(str) {return str.split("").reverse().join("")} 

/// Get the EasyJS ASCII
pub fn easyjs_ascii() {
    return "    ___       ___       ___       ___            ___       ___   
   /\\  \\     /\\  \\     /\\  \\     /\\__\\          /\\  \\     /\\  \\  
  /::\\  \\   /::\\  \\   /::\\  \\   |::L__L        _\\:\\  \\   /::\\  \\ 
 /::\\:\\__\\ /::\\:\\__\\ /\\:\\:\\__\\  |:::\\__\\      /\\/::\\__\\ /\\:\\:\\__\\
 \\:\\:\\/  / \\/\\::/  / \\:\\:\\/__/  /:;;/__/      \\::/\\/__/ \\:\\:\\/__/
  \\:\\/  /    /:/  /   \\::/  /   \\/__/          \\/__/     \\::/  / 
   \\/__/     \\/__/     \\/__/                              \\/__/  "
}"##;
const STD: &str = r##"// Get the last element of an array
macro last(array) {
    array[array.length - 1]
}

macro print(msg) {
    console.log(msg)
}

// Get the first element of an array
macro first(array) {
    array[0]
}

macro expect(method, error_msg) {
    javascript{
        try {
            method();
        } catch (e) {
            error_msg;
        }
    }
}"##;
const TYPES: &str = r##"/// A controlled Integer value.
/// @param {number|string} Ideally a value that is already a number. Does also work 
/// with strings but uses similar unpredictability as a normal Number() constructor.
///
/// @returns {Int} a special kind of number that only has it's integer value.
pub struct Int(Number) {
   fn new(value, round) {
     var nv = value
     // ensure value is INT
     if not Number.isInteger(value) {
        if round {
          nv = Math.round(value)
        } else {
          nv = Math.floor(value)
        }
     }
     super(nv)
   }
}
"##;
const RANDOM: &str = r##"// EasyJS implementation of random.uniform from Python.
pub fn uniform(a,b) {
    return Math.random() * (b - a + 1) + a
}

pub fn choice(array) {
    array[Math.floor(Math.random() * array.length)]
}

pub fn normal(mean, std_dev) {
    u1 = Math.random()
    u2 = Math.random()
    z0 = Math.sqrt(-2.0 * Math.log(u1)) * Math.cos(2.0 * Math.PI * u2) // Box-Muller transform
    return z0 * std_dev + mean
}

/// Shuffle an array randomly.
pub fn shuffle(arr) {
    return arr.slice().sort(fn() {
        return Math.random() - 0.5
    })
}

/// Get a random number from min max
pub random_number = fn(min, max) {return Math.floor(Math.random() * (max - min + 1) + min)}

/// Get a random hex color
pub random_hex_color = fn() {return "#${Math.random().toString(16).slice(2, 8).padEnd(6, '0')}"}

/// Get a Random boolean
pub random_bool = fn() {return Math.random() >= 0.5}

"##;
const MATH: &str = r##"pub fn radians(degrees) {
    javascript{
        return degrees * (Math.PI / 180);
    }
}

// Calculate the percentage in EasyJS.
pub fn calculate_percent(value,total) {
    Math.round((value / total) * 100)
}
"##;
const DATE: &str = r##"/// Get the days between 2 dates
pub days_between_dates = fn(d1, d2) { return Math.ceil(Math.abs(date1 - date2) / (1000 * 60 * 60 * 24)) }

/// Get the weekday of a date.
pub get_week_day = fn(d) { return d.toLocaleString('en-US', {weekday: 'long'}) }

/// Is a date a weekend?
pub is_weekend = fn(d) {return [5,6].indexOf(d.getDay()) != -1}"##;
const DOM: &str = r##"// ! This can only be used in the browser.

// shorthand for document.
dom = {
    create_element: fn(name) {
        return document.createElement(name)
    }

    select_all: fn(query) {
        return document.querySelectorAll(query)
    }

    add_to_body: fn(node) {
        document.body.appendChild(node)
    } 

    remove_from_body: fn(node) {
        document.body.removeChild(node)
    }
}"##;
const WASM: &str = r##"/// the EasyWasm library
pub struct EasyWasm {
    fn compile(path_to_wasm) {}
}"##;
const HTTP: &str = r##""##;

/// Load a STD library from EasyJS version 0.2.4, or an empty string if not found.
pub fn load_std(name: &str) -> String {
match name {
"ui" => UI,
"builtins" => BUILTINS,
"std" => STD,
"types" => TYPES,
"random" => RANDOM,
"math" => MATH,
"date" => DATE,
"dom" => DOM,
"wasm" => WASM,
"http" => HTTP,
_ => "",
}.to_string()}