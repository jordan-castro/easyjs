// EasyJS STD version 0.1.7
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
const HTTP: &str = "// Make a get request using the Fetch api.
async fn get(url, headers, body) {
    return fetch(url, headers, body)
}

async fn post(url, headers, body) {
    return fetch(url, headers, body)
}

some := expect(get(\"url\"), \"Error getting URL\")

let asd12dsamc = {
    success: false,
    result: null
};
try {
    asd12dsamc['result'] = get(url);
    asd12dsamc['success'] = true
} catch (e) {
    console.error(e)
    console.error(\"Error getting url\")
}

const some = asd12dsamc['result']";
const JSON: &str = "to_json := fn(str) { return JSON.parse(str); }";
const STD: &str = "// Print a value to the console.
fn $print(val) {
    // This is a compiler macro because there is no return.
    console.log(val)
}

// Get the last element of an array
fn $last(array) {
    // this is a compiler macro because there is no return.
    array[array.length - 1]
}

// Get the first element of an array
fn $first(array) {
    // this is a compiler macro because there is no return.
    array[0]
}

// Instantiate a new struct in JS
fn $new(object_name) {
    // this is a compiler macro because there is no return
    javascript{
        new object_name()
    }
}

fn $expect(method, error_msg) {
    javascript{
        try {
            method();
        } catch (e) {
            error_msg;
        }
    }
}";
const WASM: &str = "";

/// Load a STD library from EasyJS version 0.1.7, or an empty string if not found.
pub fn load_std(name: &str) -> String {
match name {
"dom" => DOM,
"easy_wasm" => EASY_WASM,
"expect" => EXPECT,
"http" => HTTP,
"json" => JSON,
"std" => STD,
"wasm" => WASM,
_ => "",
}.to_string()}