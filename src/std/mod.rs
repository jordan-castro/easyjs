// EasyJS STD version 0.1.4
const JSON: &str = "to_json := fn(str) { return JSON.parse(str); }";
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
const WASM: &str = "";
const PRINT: &str = "print := console.log";
const HTTP: &str = "// Make a get request using the Fetch api.
async fn get(url, headers, body) {
    return fetch(url, headers, body)
}

async fn post(url, headers, body) {
    return fetch(url, headers, body)
}";

/// Load a STD library from EasyJS version 0.1.4, or an empty string if not found.
pub fn load_std(name: &str) -> String {
match name {
"json" => JSON,
"dom" => DOM,
"easy_wasm" => EASY_WASM,
"wasm" => WASM,
"print" => PRINT,
"http" => HTTP,
_ => "",
}.to_string()}