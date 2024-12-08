// EasyJS STD version 0.2.1
const BUILTINS: &str = "export fn int_range(start, end) {
    res = []
    // the .. works because this is a for loop...
    for i in start..end {
        res.push(i)
    }

    return res
}

export fn sleep(ms) {
    javascript{
        return new Promise(resolve => setTimeout(resolve, ms))
    }
}

/// Creates a range
export fn range(kwargs) {
    start = kwargs.start
    end = kwargs.end
    step = kwargs.step ?? 1

    javascript{
        return Array(Math.ceil((end - start) / step)).fill(start).map((x,y) => x + y * step)
    }
}";
const DOM: &str = "// ! This can only be used in the browser.

// shorthand for document.
dom := {
    create_element: fn (name) {
        return document.createElement(name)
    }

    select_all: fn (query) {
        return document.querySelectorAll(query)
    }

    add_to_body: fn (node) {
        document.body.appendChild(node)
    } 

    remove_from_body: fn (node) {
        document.body.removeChild(node)
    }
}";
const HTTP: &str = "";
const JSON: &str = "export to_json := fn(str) { return JSON.parse(str) }
export to_string := fn(json) { return JSON.stringify(json) }";
const MATH: &str = "export fn radians(degrees) {
    javascript{
        return degrees * (Math.PI / 180);
    }
}";
const RANDOM: &str = "// EasyJS implementation of random.uniform from Python.
export fn uniform(a,b) {
    return Math.random() * (b - a + 1) + a
}

export fn choice(array) {
    array[Math.floor(Math.random() * array.length)]
}

export fn normal(mean, std_dev) {
    u1 = Math.random()
    u2 = Math.random()
    z0 = Math.sqrt(-2.0 * Math.log(u1)) * Math.cos(2.0 * Math.PI * u2) // Box-Muller transform
    return z0 * std_dev + mean
}";
const STD: &str = "// Get the last element of an array
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
}";
const UI: &str = "// Used for creating EasyJS webapps.
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
        if @is(element, \"array\") {
            self.children.add_many(element)
        } else {
            self.children.add_one(element)
        }
    }
}";
const WASM: &str = "// the EasyWasm library
";

/// Load a STD library from EasyJS version 0.2.1, or an empty string if not found.
pub fn load_std(name: &str) -> String {
match name {
"builtins" => BUILTINS,
"dom" => DOM,
"http" => HTTP,
"json" => JSON,
"math" => MATH,
"random" => RANDOM,
"std" => STD,
"ui" => UI,
"wasm" => WASM,
_ => "",
}.to_string()}