// EasyJS STD version 0.2.0
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
const EASY_WASM: &str = "// Used for working with EasyJS's WASM worker.

import \"wasm\"

struct EasyWasm {
    static async fn load_from_file(file_path) {
        return await EasyWasm.load_from_bytes()
    }
}";
const EXPECT: &str = "fn $expect(method, error_msg, var_name) {
    var_name = null
        // using javascript because EasyJS currently does not have
        // a native try-catch feature.
        javascript{
            try {
                result = method;
                var_name = result()
            } catch (e) {
                console.error(error_msg);
            }
        }
}";
const HTTP: &str = "import \"std\"

// Make a get request using the Fetch api.
async fn get(url) {
    return fetch(url)
}

async fn post(url, headers, body) {
    return fetch(url, headers, body)
}

some := $expect(get(\"https://google.com\"), \"Error getting URL\")

console.log(some)";
const JSON: &str = "to_json := fn(str) { return JSON.parse(str); }";
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
const WASM: &str = "";

/// Load a STD library from EasyJS version 0.2.0, or an empty string if not found.
pub fn load_std(name: &str) -> String {
    match name {
        "dom" => DOM,
        "easy_wasm" => EASY_WASM,
        "expect" => EXPECT,
        "http" => HTTP,
        "json" => JSON,
        "std" => STD,
        "ui" => UI,
        "wasm" => WASM,
        _ => "",
    }
    .to_string()
}
