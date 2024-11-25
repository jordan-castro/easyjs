use crate::{commands::compile::compile, utils::reader::read_file};
use crate::std::load_std;
use super::transpile::Transpiler;

/// Import EasyJS STD lib.
pub fn import_std_lib(ts: &mut Transpiler, lib_path: &str) -> String {
    // Make sure this is a file of ours.
    let contents = load_std(&lib_path);
    if contents.len() > 0 {
        return ts.transpile_from_string(contents);
    }

    String::new()
}

/// Import EasyJS modules.
pub fn import_easy_js(ts: &mut Transpiler, file_path: &str) -> String {
    // first read the file contnst
    let contents = read_file(file_path);
    if contents.len() > 0 {
        return ts.transpile_from_string(contents);
    }

    String::new()
}

/// Import WASM modules
pub fn import_wasm_module(file_path: &str) -> String {
    String::new()
}

pub enum ImportType {
EasyJS,
WASM,
JavaScript,
STD
}

pub fn get_import_type(file_path: &str) -> ImportType {
    if file_path.ends_with(".ej") {
        ImportType::EasyJS
    } else if file_path.ends_with(".wasm") {
        ImportType::WASM
    } else if file_path.ends_with(".js") {
        ImportType::JavaScript
    } else {
        ImportType::STD
    }
}