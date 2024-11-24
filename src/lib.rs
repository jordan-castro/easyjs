pub mod lexer;
pub mod utils;
pub mod parser;
pub mod compiler;
pub mod repl;
pub mod commands;
pub mod std;
pub mod interpreter;
pub mod builtins;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_easy_js(input: String) -> String {
    commands::compile::compile(input, false, true)
}