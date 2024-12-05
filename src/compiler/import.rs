use crate::commands::compile::compile_module;
use crate::utils::reader::{write_file, read_file};
use crate::std::load_std;
use super::transpile::Transpiler;

fn cwd() -> String {
    std::env::current_dir().unwrap().to_str().unwrap().to_string()
}

const CACHE_DIR: &str = "./.ejc";

fn create_cache_dir() {
    if !std::path::Path::new(CACHE_DIR).exists() {
        std::fs::create_dir(CACHE_DIR).unwrap();
    }
}

fn import_core(file_path: &str, t: &mut Transpiler) -> String {
    create_cache_dir();
    // first read the file contnst
    let contents = load_std(file_path);

    // write a file
    if contents.len() == 0 {
        panic!("File does not exist: {}", file_path);
    }
    let new_file_path = format!("{}/{}.js", CACHE_DIR, file_path);
    let mut src = compile_module(contents);
    write_file(&new_file_path, &src.0);
    t.add_module(&mut src.1);
    new_file_path
}

fn import_base(file_path: &str, t: &mut Transpiler) -> String {
    create_cache_dir();
    // get cwd...
    let actual_fp = file_path.split(".").collect::<Vec<_>>().join("/");
    let contents = read_file(&format!("{}.ej", actual_fp));
    let new_fp = format!("{}/{}.js", CACHE_DIR, actual_fp);
    // compile and write
    let mut src = compile_module(contents);
    write_file(&new_fp, &src.0);
    t.add_module(&mut src.1);

    new_fp
}

fn import_js(file_path: &str) -> String {
    create_cache_dir();
    let actual_fp = file_path.split(".").collect::<Vec<_>>().join("/");
    let contents = read_file(format!("{}.js", actual_fp).as_str());
    let new_fp = format!("{}/{}.js", CACHE_DIR, file_path);
    write_file(new_fp.as_str(), contents.as_str());
    new_fp
}

/// Import the file based on the import type.
pub fn import(file_path: &str, import_type: ImportType, t: &mut Transpiler) -> String {
    match import_type {
        ImportType::Core => import_core(file_path, t),
        ImportType::Base => import_base(file_path, t),
        ImportType::JS => import_js(file_path),
        // TODO: add compiler support for strings...
        ImportType::String => file_path.to_string(),
    }
}

/// Easily grab the module name.
pub fn get_js_module_name(file_path: &str) -> String {
    // URLS...
    if file_path.starts_with("http") {
        // get the last part of the url
        let last_path = file_path.split("/").last().unwrap().to_string();
        if last_path.ends_with(".js") || last_path.ends_with(".ts") {
            return last_path.replace(".js", "").replace(".ts", "");
        }
        return last_path;
    }

    // NPM, JSR, etc
    if file_path.contains(':') {
        // this is a 'npm:fs' or a 'jsr:fs@1.0.0'
        let file_path = file_path.split(":").collect::<Vec<_>>();
        let file_path = file_path[file_path.len() - 1];

        // split by '@' for @1.0.0 
        if file_path.contains('@') {
            let file_path = file_path.split("@").collect::<Vec<_>>();
            return file_path[0].to_string();
        } else {
            // already got the lib name...
            return file_path.to_string();
        }
    }

    // does the file_path include a '.'
    if file_path.contains('.') {
        let file_path = file_path.split(".").collect::<Vec<_>>();
        let last = file_path.last().unwrap().to_string();
        if last == "js"  || last == "ts" || last == "ej" {
            return file_path[file_path.len() - 2].to_string();
        }
        return file_path[file_path.len() - 1].to_string();
    }

    file_path.to_string()
}

pub enum ImportType {
    Core,
    Base,
    JS,
    String,
}