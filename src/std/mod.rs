// EasyJS STD version 0.1.4
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
const WASM: &str = "";

/// Load a STD library from EasyJS version 0.1.4, or an empty string if not found.
pub fn load_std(name: &str) -> String {
match name {
"dom" => DOM,
"easy_wasm" => EASY_WASM,
"http" => HTTP,
"json" => JSON,
"wasm" => WASM,
_ => "",
}.to_string()}