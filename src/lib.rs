pub mod lexer;
pub mod utils;
pub mod parser;
pub mod compiler;
pub mod repl;
pub mod commands;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_easy_js(input: String) -> String {
    commands::compile::compile(input, false)
}